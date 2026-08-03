//! HTTP adapter that calls live TMF Open APIs on `bss-oss-server`.

use async_trait::async_trait;
use chrono::{Duration, Utc};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::error::{ProductError, ProductResult};
use crate::gateway::{BalanceRecord, TmfGateway};
use crate::products::Money;

/// Calls the BSS/OSS server over HTTP with a JWT bearer token.
#[derive(Clone)]
pub struct HttpTmfGateway {
    base_url: String,
    token: String,
    client: Client,
}

impl HttpTmfGateway {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
            client: Client::new(),
        }
    }

    /// `BSS_OSS_BASE_URL` (default `http://127.0.0.1:8080`) + `BSS_OSS_TOKEN`.
    pub fn from_env() -> Option<Self> {
        let token = std::env::var("BSS_OSS_TOKEN").ok()?;
        if token.trim().is_empty() {
            return None;
        }
        let base = std::env::var("BSS_OSS_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".into());
        Some(Self::new(base, token))
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(&self, path: &str) -> ProductResult<(StatusCode, Option<T>)> {
        let res = self
            .client
            .get(self.url(path))
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| ProductError::Other(e.into()))?;
        let status = res.status();
        if status == StatusCode::NOT_FOUND {
            return Ok((status, None));
        }
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(ProductError::Other(anyhow::anyhow!(
                "GET {path} failed ({status}): {body}"
            )));
        }
        let value = res
            .json::<T>()
            .await
            .map_err(|e| ProductError::Other(e.into()))?;
        Ok((status, Some(value)))
    }

    async fn post_json<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> ProductResult<T> {
        let res = self
            .client
            .post(self.url(path))
            .bearer_auth(&self.token)
            .json(body)
            .send()
            .await
            .map_err(|e| ProductError::Other(e.into()))?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(ProductError::Other(anyhow::anyhow!(
                "POST {path} failed ({status}): {text}"
            )));
        }
        res.json::<T>()
            .await
            .map_err(|e| ProductError::Other(e.into()))
    }

    async fn find_balance(
        &self,
        party_id: Uuid,
        balance_type: &str,
        unit: &str,
    ) -> ProductResult<Option<IdMoney>> {
        let (_status, list) = self
            .get_json::<Vec<IdMoney>>("/tmf-api/prepayBalanceManagement/v4/prepayBalance")
            .await?;
        Ok(list.and_then(|items| {
            items.into_iter().find(|b| {
                b.party_id == Some(party_id)
                    && b.balance_type.eq_ignore_ascii_case(balance_type)
                    && b.remaining_value.unit == unit
            })
        }))
    }
}

#[derive(Debug, Deserialize)]
struct IdOnly {
    id: Uuid,
}

#[derive(Debug, Deserialize)]
struct IdMoney {
    id: Uuid,
    #[serde(default)]
    party_id: Option<Uuid>,
    #[serde(default)]
    balance_type: String,
    remaining_value: MoneyDto,
}

#[derive(Debug, Deserialize)]
struct MoneyDto {
    value: f64,
    unit: String,
}

fn map_balance_type(s: &str) -> &'static str {
    match s.to_uppercase().as_str() {
        "DATA" => "DATA",
        "VOICE" => "VOICE",
        "SMS" => "SMS",
        "OTHER" => "OTHER",
        _ => "MONETARY",
    }
}

#[async_trait]
impl TmfGateway for HttpTmfGateway {
    async fn ensure_customer(&self, customer_id: Uuid) -> ProductResult<()> {
        let path = format!("/tmf-api/customerManagement/v4/customer/{customer_id}");
        let (status, _) = self.get_json::<serde_json::Value>(&path).await?;
        if status == StatusCode::NOT_FOUND {
            return Err(ProductError::CustomerNotFound(customer_id.to_string()));
        }
        Ok(())
    }

    async fn ensure_party(&self, party_id: Uuid) -> ProductResult<()> {
        let path = format!("/tmf-api/partyManagement/v4/party/{party_id}");
        let (status, _) = self.get_json::<serde_json::Value>(&path).await?;
        if status == StatusCode::NOT_FOUND {
            return Err(ProductError::PartyNotFound(party_id.to_string()));
        }
        Ok(())
    }

    async fn create_payment(&self, customer_id: Uuid, amount: &Money) -> ProductResult<Uuid> {
        let body = json!({
            "name": format!("topup-{}", customer_id),
            "description": "Real-time top-up via telecom-product-engine",
            "amount": { "value": amount.value, "unit": amount.unit },
            "relatedParty": [{
                "id": customer_id,
                "name": customer_id.to_string(),
                "role": "customer"
            }]
        });
        let created: IdOnly = self
            .post_json("/tmf-api/payment/v4/payment", &body)
            .await
            .map_err(|e| ProductError::PaymentFailed(e.to_string()))?;
        Ok(created.id)
    }

    async fn credit_balance(
        &self,
        party_id: Uuid,
        amount: &Money,
        balance_type: &str,
    ) -> ProductResult<BalanceRecord> {
        let bt = map_balance_type(balance_type);
        if let Some(existing) = self.find_balance(party_id, bt, &amount.unit).await? {
            let body = json!({
                "delta": { "value": amount.value, "unit": amount.unit },
                "reason": "product-engine credit"
            });
            let path = format!(
                "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{}/adjust",
                existing.id
            );
            let updated: IdMoney = self.post_json(&path, &body).await?;
            return Ok(BalanceRecord {
                id: updated.id,
                party_id,
                amount: Money {
                    value: updated.remaining_value.value,
                    unit: updated.remaining_value.unit,
                },
                balance_type: bt.into(),
            });
        }

        let created: IdMoney = self
            .post_json(
                "/tmf-api/prepayBalanceManagement/v4/prepayBalance",
                &json!({
                    "name": format!("{bt}-{party_id}"),
                    "balance_type": bt,
                    "remaining_value": { "value": amount.value, "unit": amount.unit },
                    "party_id": party_id
                }),
            )
            .await?;
        Ok(BalanceRecord {
            id: created.id,
            party_id,
            amount: Money {
                value: created.remaining_value.value,
                unit: created.remaining_value.unit,
            },
            balance_type: bt.into(),
        })
    }

    async fn debit_balance(
        &self,
        party_id: Uuid,
        amount: &Money,
        balance_type: &str,
    ) -> ProductResult<BalanceRecord> {
        let bt = map_balance_type(balance_type);
        let existing = self
            .find_balance(party_id, bt, &amount.unit)
            .await?
            .ok_or_else(|| ProductError::InsufficientBalance {
                needed: amount.value,
                available: 0.0,
                unit: amount.unit.clone(),
            })?;

        if existing.remaining_value.value < amount.value {
            return Err(ProductError::InsufficientBalance {
                needed: amount.value,
                available: existing.remaining_value.value,
                unit: amount.unit.clone(),
            });
        }

        let body = json!({
            "delta": { "value": -amount.value, "unit": amount.unit },
            "reason": "product-engine debit"
        });
        let path = format!(
            "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{}/adjust",
            existing.id
        );
        let updated: IdMoney = self.post_json(&path, &body).await?;
        Ok(BalanceRecord {
            id: updated.id,
            party_id,
            amount: Money {
                value: updated.remaining_value.value,
                unit: updated.remaining_value.unit,
            },
            balance_type: bt.into(),
        })
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
        name: &str,
        duration_minutes: u32,
    ) -> ProductResult<(Uuid, Uuid, chrono::DateTime<Utc>)> {
        let expires = Utc::now() + Duration::minutes(duration_minutes as i64);
        let slice: IdOnly = self
            .post_json(
                "/tmf-api/sliceManagement/v4/networkSlice",
                &json!({
                    "name": name,
                    "description": format!("Turbo boost for {duration_minutes} minutes"),
                    "slice_type": "ENHANCED_MOBILE_BROADBAND",
                    "activation_date": Utc::now().to_rfc3339(),
                    "sla_parameters": {
                        "min_throughput_mbps": 500,
                        "max_latency_ms": 20
                    }
                }),
            )
            .await
            .map_err(|e| ProductError::ActivationFailed(e.to_string()))?;

        let activation: IdOnly = self
            .post_json(
                "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation",
                &json!({
                    "name": format!("turbo-{name}"),
                    "description": "On-demand speed upgrade",
                    "configuration": [
                        { "name": "slice_id", "value": slice.id.to_string() },
                        { "name": "expires_at", "value": expires.to_rfc3339() }
                    ]
                }),
            )
            .await
            .map_err(|e| ProductError::ActivationFailed(e.to_string()))?;

        Ok((slice.id, activation.id, expires))
    }

    async fn create_bnpl_account(
        &self,
        party_id: Uuid,
        device_name: &str,
        total: &Money,
        installments: u32,
    ) -> ProductResult<(Uuid, String, Money)> {
        if installments == 0 {
            return Err(ProductError::Validation(
                "installments must be >= 1".into(),
            ));
        }
        let label = format!("BNPL-{device_name}-{installments}x");
        let account: IdOnly = self
            .post_json(
                "/tmf-api/accountManagement/v4/billingAccount",
                &json!({
                    "name": label,
                    "description": format!("BNPL total {} {}", total.value, total.unit),
                    "account_type": "BNPL",
                    "related_party": [{
                        "id": party_id,
                        "name": party_id.to_string(),
                        "role": "owner"
                    }]
                }),
            )
            .await?;
        let installment = Money {
            value: total.value / installments as f64,
            unit: total.unit.clone(),
        };
        Ok((account.id, label, installment))
    }

    async fn issue_identity(&self, party_id: Uuid, login: &str) -> ProductResult<(Uuid, String)> {
        let identity: IdOnly = self
            .post_json(
                "/tmf-api/identityManagement/v4/identity",
                &json!({
                    "name": login,
                    "party_id": party_id,
                    "identity_type": "login",
                    "credential": [{
                        "credential_type": "PASSWORD",
                        "credential_value": "change-me"
                    }]
                }),
            )
            .await
            .map_err(|e| ProductError::Identity(e.to_string()))?;
        Ok((identity.id, format!("{login}@telecom.local")))
    }
}
