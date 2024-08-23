// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="10")]
    pub token_splitting_payments: ::prost::alloc::vec::Vec<TokenSplittingPayment>,
    #[prost(message, repeated, tag="20")]
    pub regular_driver_payments: ::prost::alloc::vec::Vec<RegularDriverPayment>,
    #[prost(message, repeated, tag="30")]
    pub no_split_payments: ::prost::alloc::vec::Vec<NoSplitPayment>,
    #[prost(message, repeated, tag="50")]
    pub ai_trainer_payments: ::prost::alloc::vec::Vec<AiTrainerPayment>,
    #[prost(message, repeated, tag="51")]
    pub operational_payments: ::prost::alloc::vec::Vec<OperationalPayment>,
    #[prost(message, repeated, tag="60")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
    #[prost(message, repeated, tag="70")]
    pub mints: ::prost::alloc::vec::Vec<Mint>,
    #[prost(message, repeated, tag="80")]
    pub burns: ::prost::alloc::vec::Vec<Burn>,
    #[prost(message, repeated, tag="120")]
    pub initialized_account: ::prost::alloc::vec::Vec<InitializedAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenSplittingPayment {
    #[prost(message, optional, tag="1")]
    pub manager_mint: ::core::option::Option<Mint>,
    #[prost(message, optional, tag="2")]
    pub driver_mint: ::core::option::Option<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegularDriverPayment {
    #[prost(message, optional, tag="1")]
    pub mint: ::core::option::Option<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoSplitPayment {
    #[prost(message, optional, tag="1")]
    pub mint: ::core::option::Option<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiTrainerPayment {
    #[prost(message, optional, tag="1")]
    pub mint: ::core::option::Option<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationalPayment {
    #[prost(message, optional, tag="1")]
    pub mint: ::core::option::Option<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    #[prost(message, optional, tag="1")]
    pub mint: ::core::option::Option<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub amount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub amount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burn {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub amount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializedAccount {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
