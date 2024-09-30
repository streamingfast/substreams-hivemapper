use crate::pb::hivemapper::types::v1::{
    entity, Burn, Entity, InitializedAccount, MapCreate, Mint, Payment, Transaction, Transfer,
};

impl Entity {
    pub fn new_payment(payment: Payment) -> Self {
        Entity {
            item: Some(entity::Item::Payment(payment)),
        }
    }

    pub fn new_map_create(map_create: MapCreate) -> Self {
        Entity {
            item: Some(entity::Item::MapCreate(map_create)),
        }
    }

    pub fn new_transfer(transfer: Transfer) -> Self {
        Entity {
            item: Some(entity::Item::Transfers(transfer)),
        }
    }

    pub fn new_mint(mint: Mint) -> Self {
        Entity {
            item: Some(entity::Item::Mints(mint)),
        }
    }

    pub fn new_burn(burn: Burn) -> Self {
        Entity {
            item: Some(entity::Item::Burns(burn)),
        }
    }

    pub fn new_initialized_account(initialized_account: InitializedAccount) -> Self {
        Entity {
            item: Some(entity::Item::InitializedAccount(initialized_account)),
        }
    }
}

impl Mint {
    pub fn new(timestamp: i64, to: String, amount: f64) -> Self {
        Mint { timestamp, to, amount }
    }
}

impl Burn {
    pub fn new(timestamp: i64, from: String, amount: f64) -> Self {
        Burn {
            timestamp,
            from,
            amount,
        }
    }
}

impl Transaction {
    pub fn new(hash: String) -> Self {
        Transaction {
            hash,
            entities: Default::default(),
        }
    }
}
