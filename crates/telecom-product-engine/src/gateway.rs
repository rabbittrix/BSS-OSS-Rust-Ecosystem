//! Gateway abstraction over TMF domain operations

use async_trait::async_trait;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::error::{ProductError, ProductResult};
use crate::products::Money;

#[derive(Debug, Clone)]
pub struct BalanceRecord {
    pub id: Uuid,
    pub party_id: Uuid,
    pub amount: Money,
    pub balance_type: String,
}

#[async_trait]
pub trait TmfGateway: Send + Sync {
    async fn ensure_customer(&self, customer_id: Uuid) -> ProductResult<()>;
    async fn ensure_party(&self, party_id: Uuid) -> ProductResult<()>;

    async fn create_payment(&self, customer_id: Uuid, amount: &Money) -> ProductResult<Uuid>;
    async fn credit_balance(
        &self,
        party_id: Uuid,
        amount: &Money,
        balance_type: &str,
    ) -> ProductResult<BalanceRecord>;
    async fn debit_balance(
        &self,
        party_id: Uuid,
        amount: &Money,
        balance_type: &str,
    ) -> ProductResult<BalanceRecord>;
    async fn get_or_create_data_balance(&self, party_id: Uuid) -> ProductResult<BalanceRecord>;

    async fn activate_turbo_slice(
        &self,
        customer_id: Uuid,
        name: &str,
        duration_minutes: u32,
    ) -> ProductResult<(Uuid, Uuid, chrono::DateTime<Utc>)>;

    async fn create_bnpl_account(
        &self,
        party_id: Uuid,
        device_name: &str,
        total: &Money,
        installments: u32,
    ) -> ProductResult<(Uuid, String, Money)>;

    async fn issue_identity(&self, party_id: Uuid, login: &str) -> ProductResult<(Uuid, String)>;
}

/// In-memory gateway for local dashboard / unit tests (no HTTP).
#[derive(Default, Clone)]
pub struct InMemoryGateway {
    inner: Arc<RwLock<InMemoryState>>,
}

#[derive(Default)]
struct InMemoryState {
    customers: HashMap<Uuid, bool>,
    parties: HashMap<Uuid, bool>,
    balances: HashMap<Uuid, BalanceRecord>,
    /// party_id -> balance ids
    party_balances: HashMap<Uuid, Vec<Uuid>>,
}

#[async_trait]
impl TmfGateway for InMemoryGateway {
    async fn ensure_customer(&self, customer_id: Uuid) -> ProductResult<()> {
        self.inner.write().await.customers.insert(customer_id, true);
        Ok(())
    }

    async fn ensure_party(&self, party_id: Uuid) -> ProductResult<()> {
        self.inner.write().await.parties.insert(party_id, true);
        Ok(())
    }

    async fn create_payment(&self, _customer_id: Uuid, amount: &Money) -> ProductResult<Uuid> {
        if amount.value <= 0.0 {
            return Err(ProductError::PaymentFailed(
                "amount must be positive".into(),
            ));
        }
        Ok(Uuid::new_v4())
    }

    async fn credit_balance(
        &self,
        party_id: Uuid,
        amount: &Money,
        balance_type: &str,
    ) -> ProductResult<BalanceRecord> {
        let mut state = self.inner.write().await;
        let existing = state
            .party_balances
            .get(&party_id)
            .and_then(|ids| {
                ids.iter().find_map(|id| {
                    state.balances.get(id).filter(|b| b.balance_type == balance_type && b.amount.unit == amount.unit)
                })
            })
            .cloned();

        if let Some(mut bal) = existing {
            bal.amount.value += amount.value;
            state.balances.insert(bal.id, bal.clone());
            return Ok(bal);
        }

        let id = Uuid::new_v4();
        let record = BalanceRecord {
            id,
            party_id,
            amount: amount.clone(),
            balance_type: balance_type.into(),
        };
        state.balances.insert(id, record.clone());
        state.party_balances.entry(party_id).or_default().push(id);
        Ok(record)
    }

    async fn debit_balance(
        &self,
        party_id: Uuid,
        amount: &Money,
        balance_type: &str,
    ) -> ProductResult<BalanceRecord> {
        let mut state = self.inner.write().await;
        let bal_id = state
            .party_balances
            .get(&party_id)
            .and_then(|ids| {
                ids.iter().find(|id| {
                    state.balances.get(id).is_some_and(|b| {
                        b.balance_type == balance_type && b.amount.unit == amount.unit
                    })
                })
            })
            .copied()
            .ok_or_else(|| ProductError::InsufficientBalance {
                needed: amount.value,
                available: 0.0,
                unit: amount.unit.clone(),
            })?;

        let bal = state.balances.get_mut(&bal_id).unwrap();
        if bal.amount.value < amount.value {
            return Err(ProductError::InsufficientBalance {
                needed: amount.value,
                available: bal.amount.value,
                unit: amount.unit.clone(),
            });
        }
        bal.amount.value -= amount.value;
        Ok(bal.clone())
    }

    async fn get_or_create_data_balance(&self, party_id: Uuid) -> ProductResult<BalanceRecord> {
        self.credit_balance(
            party_id,
            &Money {
                value: 0.0,
                unit: "GB".into(),
            },
            "DATA",
        )
        .await
    }

    async fn activate_turbo_slice(
        &self,
        _customer_id: Uuid,
        _name: &str,
        duration_minutes: u32,
    ) -> ProductResult<(Uuid, Uuid, chrono::DateTime<Utc>)> {
        if duration_minutes == 0 {
            return Err(ProductError::ActivationFailed(
                "duration must be > 0".into(),
            ));
        }
        let expires = Utc::now() + Duration::minutes(duration_minutes as i64);
        Ok((Uuid::new_v4(), Uuid::new_v4(), expires))
    }

    async fn create_bnpl_account(
        &self,
        _party_id: Uuid,
        device_name: &str,
        total: &Money,
        installments: u32,
    ) -> ProductResult<(Uuid, String, Money)> {
        if installments == 0 {
            return Err(ProductError::Validation(
                "installments must be >= 1".into(),
            ));
        }
        let installment = Money {
            value: total.value / installments as f64,
            unit: total.unit.clone(),
        };
        Ok((
            Uuid::new_v4(),
            format!("BNPL-{device_name}-{installments}x"),
            installment,
        ))
    }

    async fn issue_identity(&self, _party_id: Uuid, login: &str) -> ProductResult<(Uuid, String)> {
        if login.trim().is_empty() {
            return Err(ProductError::Identity("login required".into()));
        }
        Ok((Uuid::new_v4(), format!("{login}@telecom.local")))
    }
}
