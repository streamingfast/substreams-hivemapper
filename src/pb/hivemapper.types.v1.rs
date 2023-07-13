// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub token_splitting_payments: ::prost::alloc::vec::Vec<TokenSplittingPayment>,
    #[prost(message, repeated, tag="2")]
    pub driver_payments: ::prost::alloc::vec::Vec<DriverPayment>,
    #[prost(message, repeated, tag="3")]
    pub driver_foundation_payments: ::prost::alloc::vec::Vec<DriverFoundationPayment>,
    #[prost(message, repeated, tag="4")]
    pub ai_trainer_payments: ::prost::alloc::vec::Vec<AiTrainerPayment>,
    #[prost(message, repeated, tag="5")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
    #[prost(message, repeated, tag="6")]
    pub mints: ::prost::alloc::vec::Vec<Mint>,
    #[prost(message, repeated, tag="7")]
    pub burns: ::prost::alloc::vec::Vec<Burn>,
    #[prost(message, repeated, tag="8")]
    pub transfer_checks: ::prost::alloc::vec::Vec<TransferChecked>,
    #[prost(message, repeated, tag="9")]
    pub mint_to_checks: ::prost::alloc::vec::Vec<MintToChecked>,
    #[prost(message, repeated, tag="10")]
    pub burn_checks: ::prost::alloc::vec::Vec<BurnChecked>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenSplittingPayment {
    #[prost(message, optional, tag="1")]
    pub manager: ::core::option::Option<Payment>,
    #[prost(message, optional, tag="2")]
    pub driver: ::core::option::Option<Payment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriverPayment {
    #[prost(message, optional, tag="1")]
    pub payment: ::core::option::Option<Payment>,
    #[prost(enumeration="driver_payment::DriverType", tag="2")]
    pub r#type: i32,
}
/// Nested message and enum types in `DriverPayment`.
pub mod driver_payment {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DriverType {
        Unset = 0,
        /// Someone not part of a fleet
        Regular = 1,
        /// We can't know if it's a manager or a fleet member at this point
        NoSplit = 2,
    }
    impl DriverType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DriverType::Unset => "UNSET",
                DriverType::Regular => "REGULAR",
                DriverType::NoSplit => "NO_SPLIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSET" => Some(Self::Unset),
                "REGULAR" => Some(Self::Regular),
                "NO_SPLIT" => Some(Self::NoSplit),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriverFoundationPayment {
    #[prost(message, optional, tag="1")]
    pub payment: ::core::option::Option<Payment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiTrainerPayment {
    #[prost(message, optional, tag="1")]
    pub payment: ::core::option::Option<Payment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
    #[prost(string, tag="2")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
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
    #[prost(int64, tag="5")]
    pub amount: i64,
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
    #[prost(int64, tag="5")]
    pub amount: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferChecked {
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
    #[prost(int32, tag="6")]
    pub decimals: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToChecked {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub amount: i64,
    #[prost(int32, tag="6")]
    pub decimals: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnChecked {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub amount: i64,
    #[prost(int32, tag="6")]
    pub decimals: i32,
}
// @@protoc_insertion_point(module)
