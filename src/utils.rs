use crate::pb::sol::transactions::v1::Transactions;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

/// Iterates over successful transactions in given block and take ownership.
pub fn transactions_owned(transactions: Transactions) -> impl Iterator<Item = ConfirmedTransaction> {
    transactions.transactions.into_iter().filter(|trx| -> bool {
        if let Some(meta) = &trx.meta {
            return meta.err.is_none();
        }
        false
    })
}
