//! ProductService trait — public orchestration API for the dashboard / Tauri layer

use async_trait::async_trait;

use crate::error::ProductResult;
use crate::events::{LogicStep, ProductEvent};
use crate::products::*;
use uuid::Uuid;

#[async_trait]
pub trait ProductService: Send + Sync {
    async fn real_time_topup(&self, req: TopUpRequest) -> ProductResult<TopUpResult>;
    async fn turbo_boost(&self, req: TurboBoostRequest) -> ProductResult<TurboBoostResult>;
    async fn data_wallet_transfer(
        &self,
        req: DataWalletTransferRequest,
    ) -> ProductResult<DataWalletTransferResult>;
    async fn bnpl_device(&self, req: BnplRequest) -> ProductResult<BnplResult>;
    async fn issue_identity(&self, req: IdentityIssueRequest) -> ProductResult<IdentityIssueResult>;

    /// Demo/helper: credit a DATA (or other) balance for a party.
    async fn seed_balance(
        &self,
        party_id: Uuid,
        amount: Money,
        balance_type: &str,
    ) -> ProductResult<Uuid>;

    /// Recent orchestration events for the dashboard activity feed.
    async fn recent_events(&self, limit: usize) -> ProductResult<Vec<ProductEvent>>;

    /// Recent Live Logic Viewer steps (newest last). Optionally filter by flow.
    async fn recent_logic_steps(
        &self,
        limit: usize,
        flow_id: Option<Uuid>,
    ) -> ProductResult<Vec<LogicStep>>;
}
