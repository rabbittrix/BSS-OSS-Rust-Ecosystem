//! ProductOrchestrator — default ProductService implementation

use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::error::ProductResult;
use crate::events::{LogicStep, LogicStepStatus, ProductEvent, ProductEventKind};
use crate::gateway::TmfGateway;
use crate::products::*;
use crate::service::ProductService;

pub struct ProductOrchestrator<G: TmfGateway> {
    gateway: G,
    events: Arc<RwLock<Vec<ProductEvent>>>,
    logic_steps: Arc<RwLock<Vec<LogicStep>>>,
    logic_tx: broadcast::Sender<LogicStep>,
    /// Pace between TMF steps so the Live Logic Viewer can animate in real time.
    live_delay: Duration,
}

impl<G: TmfGateway> ProductOrchestrator<G> {
    pub fn new(gateway: G) -> Self {
        let (logic_tx, _) = broadcast::channel(256);
        Self {
            gateway,
            events: Arc::new(RwLock::new(Vec::new())),
            logic_steps: Arc::new(RwLock::new(Vec::new())),
            logic_tx,
            live_delay: Duration::from_millis(350),
        }
    }

    /// Subscribe to live TMF call steps (for Tauri event bridging).
    pub fn subscribe_logic(&self) -> broadcast::Receiver<LogicStep> {
        self.logic_tx.subscribe()
    }

    async fn push(&self, event: ProductEvent) {
        let mut events = self.events.write().await;
        events.push(event);
        if events.len() > 500 {
            let drain = events.len() - 500;
            events.drain(0..drain);
        }
    }

    async fn emit_logic(&self, step: LogicStep) {
        {
            let mut steps = self.logic_steps.write().await;
            steps.push(step.clone());
            if steps.len() > 1000 {
                let drain = steps.len() - 1000;
                steps.drain(0..drain);
            }
        }
        let _ = self.logic_tx.send(step);
        if !self.live_delay.is_zero() {
            tokio::time::sleep(self.live_delay).await;
        }
    }

    async fn step(
        &self,
        flow_id: Uuid,
        seq: u32,
        product: &str,
        tmf: &str,
        method: &str,
        path: &str,
        status: LogicStepStatus,
        detail: impl Into<String>,
    ) {
        self.emit_logic(LogicStep::new(
            flow_id, seq, product, tmf, method, path, status, detail,
        ))
        .await;
    }
}

#[async_trait]
impl<G: TmfGateway + 'static> ProductService for ProductOrchestrator<G> {
    async fn real_time_topup(&self, req: TopUpRequest) -> ProductResult<TopUpResult> {
        let flow_id = Uuid::new_v4();
        let product = "real_time_topup";

        self.push(ProductEvent::new(
            ProductEventKind::TopUpStarted,
            product,
            format!("Top-up {} {}", req.amount.value, req.amount.unit),
        ))
        .await;

        self.step(
            flow_id,
            1,
            product,
            "FLOW",
            "START",
            "/products/real-time-topup",
            LogicStepStatus::Info,
            format!("channel={}", req.channel),
        )
        .await;

        self.step(
            flow_id,
            2,
            product,
            "TMF629",
            "GET",
            &format!("/tmf-api/customerManagement/v4/customer/{}", req.customer_id),
            LogicStepStatus::Started,
            "Validate customer exists",
        )
        .await;
        self.gateway.ensure_customer(req.customer_id).await?;
        self.step(
            flow_id,
            3,
            product,
            "TMF629",
            "GET",
            &format!("/tmf-api/customerManagement/v4/customer/{}", req.customer_id),
            LogicStepStatus::Succeeded,
            "Customer validated",
        )
        .await;

        self.step(
            flow_id,
            4,
            product,
            "TMF676",
            "POST",
            "/tmf-api/payment/v4/payment",
            LogicStepStatus::Started,
            format!("amount={} {}", req.amount.value, req.amount.unit),
        )
        .await;
        let payment_id = self
            .gateway
            .create_payment(req.customer_id, &req.amount)
            .await?;
        self.step(
            flow_id,
            5,
            product,
            "TMF676",
            "POST",
            "/tmf-api/payment/v4/payment",
            LogicStepStatus::Succeeded,
            format!("payment_id={payment_id}"),
        )
        .await;

        self.step(
            flow_id,
            6,
            product,
            "TMF654",
            "POST",
            &format!(
                "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{{id}}/adjust"
            ),
            LogicStepStatus::Started,
            "Credit monetary prepay balance",
        )
        .await;
        let balance = self
            .gateway
            .credit_balance(req.customer_id, &req.amount, "MONETARY")
            .await?;
        self.step(
            flow_id,
            7,
            product,
            "TMF654",
            "POST",
            &format!(
                "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{}/adjust",
                balance.id
            ),
            LogicStepStatus::Succeeded,
            format!("new_balance={} {}", balance.amount.value, balance.amount.unit),
        )
        .await;

        self.push(
            ProductEvent::new(ProductEventKind::TopUpCompleted, product, "Top-up completed")
                .with_ids([payment_id, balance.id]),
        )
        .await;

        Ok(TopUpResult {
            payment_id,
            balance_id: balance.id,
            new_balance: balance.amount,
        })
    }

    async fn turbo_boost(&self, req: TurboBoostRequest) -> ProductResult<TurboBoostResult> {
        let flow_id = Uuid::new_v4();
        let product = "turbo_boost";

        self.push(ProductEvent::new(
            ProductEventKind::TurboBoostStarted,
            product,
            format!("Boost for {} minutes", req.duration_minutes),
        ))
        .await;

        self.step(
            flow_id,
            1,
            product,
            "FLOW",
            "START",
            "/products/turbo-boost",
            LogicStepStatus::Info,
            format!(
                "Buy Turbo Boost — {} min slice '{}'",
                req.duration_minutes, req.slice_name
            ),
        )
        .await;

        self.step(
            flow_id,
            2,
            product,
            "TMF629",
            "GET",
            &format!("/tmf-api/customerManagement/v4/customer/{}", req.customer_id),
            LogicStepStatus::Started,
            "Validate subscriber before network upgrade",
        )
        .await;
        self.gateway.ensure_customer(req.customer_id).await?;
        self.step(
            flow_id,
            3,
            product,
            "TMF629",
            "GET",
            &format!("/tmf-api/customerManagement/v4/customer/{}", req.customer_id),
            LogicStepStatus::Succeeded,
            "Subscriber OK",
        )
        .await;

        self.step(
            flow_id,
            4,
            product,
            "TMF656",
            "POST",
            "/tmf-api/sliceManagement/v4/networkSlice",
            LogicStepStatus::Started,
            format!("Create temporary eMBB slice '{}'", req.slice_name),
        )
        .await;

        let (slice_id, activation_id, expires_at) = self
            .gateway
            .activate_turbo_slice(req.customer_id, &req.slice_name, req.duration_minutes)
            .await?;

        self.step(
            flow_id,
            5,
            product,
            "TMF656",
            "POST",
            "/tmf-api/sliceManagement/v4/networkSlice",
            LogicStepStatus::Succeeded,
            format!("slice_id={slice_id} state=ACTIVE"),
        )
        .await;

        self.step(
            flow_id,
            6,
            product,
            "TMF640",
            "POST",
            "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation",
            LogicStepStatus::Started,
            "Activate speed upgrade against the new slice",
        )
        .await;
        self.step(
            flow_id,
            7,
            product,
            "TMF640",
            "POST",
            "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation",
            LogicStepStatus::Succeeded,
            format!("activation_id={activation_id}"),
        )
        .await;

        self.step(
            flow_id,
            8,
            product,
            "TMF640",
            "PATCH",
            &format!(
                "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation/{activation_id}"
            ),
            LogicStepStatus::Succeeded,
            format!("expires_at={}", expires_at.to_rfc3339()),
        )
        .await;

        self.step(
            flow_id,
            9,
            product,
            "FLOW",
            "COMPLETE",
            "/products/turbo-boost",
            LogicStepStatus::Info,
            "Turbo Boost ready — temporary capacity granted",
        )
        .await;

        self.push(
            ProductEvent::new(
                ProductEventKind::TurboBoostCompleted,
                product,
                "Turbo boost activated",
            )
            .with_ids([slice_id, activation_id]),
        )
        .await;

        Ok(TurboBoostResult {
            slice_id,
            activation_id,
            expires_at,
        })
    }

    async fn data_wallet_transfer(
        &self,
        req: DataWalletTransferRequest,
    ) -> ProductResult<DataWalletTransferResult> {
        let flow_id = Uuid::new_v4();
        let product = "data_wallet";

        self.push(ProductEvent::new(
            ProductEventKind::DataTransferStarted,
            product,
            format!(
                "Transfer {} {} → {}",
                req.amount.value, req.amount.unit, req.recipient_party_id
            ),
        ))
        .await;

        self.step(
            flow_id,
            1,
            product,
            "TMF632",
            "GET",
            &format!("/tmf-api/partyManagement/v4/party/{}", req.donor_party_id),
            LogicStepStatus::Started,
            "Resolve donor party",
        )
        .await;
        self.gateway.ensure_party(req.donor_party_id).await?;
        self.gateway.ensure_party(req.recipient_party_id).await?;
        self.step(
            flow_id,
            2,
            product,
            "TMF632",
            "GET",
            &format!(
                "/tmf-api/partyManagement/v4/party/{}",
                req.recipient_party_id
            ),
            LogicStepStatus::Succeeded,
            "Donor & recipient parties OK",
        )
        .await;

        self.step(
            flow_id,
            3,
            product,
            "TMF637",
            "GET",
            "/tmf-api/productInventoryManagement/v4/product",
            LogicStepStatus::Started,
            "Locate data inventory / quota product",
        )
        .await;
        let _ = self
            .gateway
            .get_or_create_data_balance(req.donor_party_id)
            .await?;
        self.step(
            flow_id,
            4,
            product,
            "TMF637",
            "GET",
            "/tmf-api/productInventoryManagement/v4/product",
            LogicStepStatus::Succeeded,
            "Inventory linked to DATA bucket",
        )
        .await;

        self.step(
            flow_id,
            5,
            product,
            "TMF654",
            "POST",
            "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{id}/adjust",
            LogicStepStatus::Started,
            format!("Debit donor {} {}", req.amount.value, req.amount.unit),
        )
        .await;
        let donor = self
            .gateway
            .debit_balance(req.donor_party_id, &req.amount, "DATA")
            .await?;
        self.step(
            flow_id,
            6,
            product,
            "TMF654",
            "POST",
            &format!(
                "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{}/adjust",
                donor.id
            ),
            LogicStepStatus::Succeeded,
            "Donor debit applied",
        )
        .await;

        let recipient = self
            .gateway
            .credit_balance(req.recipient_party_id, &req.amount, "DATA")
            .await?;
        self.step(
            flow_id,
            7,
            product,
            "TMF654",
            "POST",
            &format!(
                "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{}/adjust",
                recipient.id
            ),
            LogicStepStatus::Succeeded,
            "Recipient credit applied",
        )
        .await;

        self.push(
            ProductEvent::new(
                ProductEventKind::DataTransferCompleted,
                product,
                "P2P data transfer completed",
            )
            .with_ids([donor.id, recipient.id]),
        )
        .await;

        Ok(DataWalletTransferResult {
            donor_balance_id: donor.id,
            recipient_balance_id: recipient.id,
            transferred: req.amount,
        })
    }

    async fn bnpl_device(&self, req: BnplRequest) -> ProductResult<BnplResult> {
        let flow_id = Uuid::new_v4();
        let product = "bnpl";

        self.push(ProductEvent::new(
            ProductEventKind::BnplStarted,
            product,
            format!("BNPL for {}", req.device_name),
        ))
        .await;

        self.step(
            flow_id,
            1,
            product,
            "TMF632",
            "GET",
            &format!("/tmf-api/partyManagement/v4/party/{}", req.party_id),
            LogicStepStatus::Started,
            "Load party for financing",
        )
        .await;
        self.gateway.ensure_party(req.party_id).await?;
        self.step(
            flow_id,
            2,
            product,
            "TMF632",
            "GET",
            &format!("/tmf-api/partyManagement/v4/party/{}", req.party_id),
            LogicStepStatus::Succeeded,
            "Party verified",
        )
        .await;

        self.step(
            flow_id,
            3,
            product,
            "TMF666",
            "POST",
            "/tmf-api/accountManagement/v4/billingAccount",
            LogicStepStatus::Started,
            format!(
                "Create BNPL billing account — {}x installments",
                req.installments
            ),
        )
        .await;
        let (account_id, agreement_label, installment_amount) = self
            .gateway
            .create_bnpl_account(
                req.party_id,
                &req.device_name,
                &req.total_amount,
                req.installments,
            )
            .await?;
        self.step(
            flow_id,
            4,
            product,
            "TMF666",
            "POST",
            "/tmf-api/accountManagement/v4/billingAccount",
            LogicStepStatus::Succeeded,
            format!("account_id={account_id} label={agreement_label}"),
        )
        .await;

        self.push(
            ProductEvent::new(ProductEventKind::BnplCompleted, product, "BNPL account ready")
                .with_ids([account_id]),
        )
        .await;

        Ok(BnplResult {
            account_id,
            agreement_label,
            installment_amount,
        })
    }

    async fn issue_identity(&self, req: IdentityIssueRequest) -> ProductResult<IdentityIssueResult> {
        let flow_id = Uuid::new_v4();
        let product = "identity_aas";

        self.step(
            flow_id,
            1,
            product,
            "TMF632",
            "GET",
            &format!("/tmf-api/partyManagement/v4/party/{}", req.party_id),
            LogicStepStatus::Started,
            "Resolve party for identity",
        )
        .await;
        self.gateway.ensure_party(req.party_id).await?;

        self.step(
            flow_id,
            2,
            product,
            "TMF669",
            "POST",
            "/tmf-api/identityManagement/v4/identity",
            LogicStepStatus::Started,
            format!("Issue identity for {}", req.login),
        )
        .await;
        let (identity_id, credential_hint) = self
            .gateway
            .issue_identity(req.party_id, &req.login)
            .await?;
        self.step(
            flow_id,
            3,
            product,
            "TMF669",
            "POST",
            "/tmf-api/identityManagement/v4/identity",
            LogicStepStatus::Succeeded,
            format!("identity_id={identity_id} hint={credential_hint}"),
        )
        .await;

        self.push(
            ProductEvent::new(
                ProductEventKind::IdentityIssued,
                product,
                format!("Identity issued for {}", req.login),
            )
            .with_ids([identity_id]),
        )
        .await;

        Ok(IdentityIssueResult {
            identity_id,
            credential_hint,
        })
    }

    async fn seed_balance(
        &self,
        party_id: Uuid,
        amount: Money,
        balance_type: &str,
    ) -> ProductResult<Uuid> {
        self.gateway.ensure_party(party_id).await?;
        let bal = self
            .gateway
            .credit_balance(party_id, &amount, balance_type)
            .await?;
        Ok(bal.id)
    }

    async fn recent_events(&self, limit: usize) -> ProductResult<Vec<ProductEvent>> {
        let events = self.events.read().await;
        let start = events.len().saturating_sub(limit);
        Ok(events[start..].iter().rev().cloned().collect())
    }

    async fn recent_logic_steps(
        &self,
        limit: usize,
        flow_id: Option<Uuid>,
    ) -> ProductResult<Vec<LogicStep>> {
        let steps = self.logic_steps.read().await;
        let filtered: Vec<LogicStep> = match flow_id {
            Some(id) => steps.iter().filter(|s| s.flow_id == id).cloned().collect(),
            None => steps.clone(),
        };
        let start = filtered.len().saturating_sub(limit);
        Ok(filtered[start..].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::InMemoryGateway;

    #[tokio::test]
    async fn topup_credits_balance() {
        let engine = ProductOrchestrator::new(InMemoryGateway::default());
        let customer = Uuid::new_v4();
        let result = engine
            .real_time_topup(TopUpRequest {
                customer_id: customer,
                amount: Money {
                    value: 25.0,
                    unit: "EUR".into(),
                },
                channel: "test".into(),
            })
            .await
            .unwrap();
        assert_eq!(result.new_balance.value, 25.0);
    }

    #[tokio::test]
    async fn turbo_boost_emits_tmf_logic_steps() {
        let mut engine = ProductOrchestrator::new(InMemoryGateway::default());
        engine.live_delay = Duration::ZERO;
        engine
            .turbo_boost(TurboBoostRequest {
                customer_id: Uuid::new_v4(),
                duration_minutes: 30,
                slice_name: "turbo".into(),
            })
            .await
            .unwrap();
        let steps = engine.recent_logic_steps(50, None).await.unwrap();
        assert!(steps.iter().any(|s| s.tmf == "TMF656"));
        assert!(steps.iter().any(|s| s.tmf == "TMF640"));
    }

    #[tokio::test]
    async fn data_wallet_requires_funds() {
        let engine = ProductOrchestrator::new(InMemoryGateway::default());
        let donor = Uuid::new_v4();
        let recipient = Uuid::new_v4();
        engine
            .gateway
            .credit_balance(
                donor,
                &Money {
                    value: 5.0,
                    unit: "GB".into(),
                },
                "DATA",
            )
            .await
            .unwrap();

        let err = engine
            .data_wallet_transfer(DataWalletTransferRequest {
                donor_party_id: donor,
                recipient_party_id: recipient,
                amount: Money {
                    value: 10.0,
                    unit: "GB".into(),
                },
            })
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            crate::error::ProductError::InsufficientBalance { .. }
        ));
    }
}
