use anyhow::Result;

use crate::models::account_enums::TonEventStatus;
use crate::models::service_id::ServiceId;
use crate::models::sqlx::TokenTransactionEventDb;
use crate::prelude::ServiceError;
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn get_token_transaction_event_by_mh(
        &self,
        message_hash: String,
        service_id: ServiceId,
        account_workchain_id: i32,
        account_hex: String,
    ) -> Result<TokenTransactionEventDb, ServiceError> {
        sqlx::query_as!(
            TokenTransactionEventDb,
            r#"
            SELECT id,
                service_id as "service_id: _",
                token_transaction_id,
                message_hash,
                account_workchain_id,
                account_hex,
                value,
                root_address,
                transaction_direction as "transaction_direction: _",
                transaction_status as "transaction_status: _",
                event_status as "event_status: _",
                created_at, updated_at
            FROM token_transaction_events
            WHERE service_id = $1 AND message_hash = $2 AND account_workchain_id = $3 AND account_hex = $4"#,
            service_id as ServiceId,
            message_hash,
            account_workchain_id,
            account_hex,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn update_event_status_of_token_transaction_event(
        &self,
        message_hash: String,
        account_workchain_id: i32,
        account_hex: String,
        event_status: TonEventStatus,
    ) -> Result<TokenTransactionEventDb, ServiceError> {
        sqlx::query_as!(
            TokenTransactionEventDb,
            r#"
            UPDATE token_transaction_events SET event_status = $1
            WHERE message_hash = $2 AND account_workchain_id = $3 AND account_hex = $4
            RETURNING id,
                service_id as "service_id: _",
                token_transaction_id,
                message_hash,
                account_workchain_id,
                account_hex,
                value,
                root_address,
                transaction_direction as "transaction_direction: _",
                transaction_status as "transaction_status: _",
                event_status as "event_status: _",
                created_at, updated_at"#,
            event_status as TonEventStatus,
            message_hash,
            account_workchain_id,
            account_hex,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn get_token_transaction_events(
        &self,
        service_id: ServiceId,
        event_status: TonEventStatus,
    ) -> Result<Vec<TokenTransactionEventDb>, ServiceError> {
        sqlx::query_as!(
            TokenTransactionEventDb,
            r#"
            SELECT id,
                service_id as "service_id: _",
                token_transaction_id,
                message_hash,
                account_workchain_id,
                account_hex,
                value,
                root_address,
                transaction_direction as "transaction_direction: _",
                transaction_status as "transaction_status: _",
                event_status as "event_status: _",
                created_at, updated_at
            FROM token_transaction_events
            WHERE service_id = $1 AND event_status = $2"#,
            service_id as ServiceId,
            event_status as TonEventStatus,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(From::from)
    }
}

#[cfg(test)]
mod test {}
