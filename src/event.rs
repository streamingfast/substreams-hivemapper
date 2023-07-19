use crate::pb::hivemapper::types::v1::{Burn, InitializedAccount, Mint, Transfer};

pub struct Event {
    pub r#type: Type,
}

pub enum Type {
    Mint(Mint),
    Burn(Burn),
    Transfer(Transfer),
    InitializeAccount(InitializedAccount),
}
