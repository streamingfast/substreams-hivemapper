// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(oneof="output::PaymentType", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub payment_type: ::core::option::Option<output::PaymentType>,
}
/// Nested message and enum types in `Output`.
pub mod output {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PaymentType {
        #[prost(message, tag="1")]
        TokenSplittingPayments(super::TokenSplittingPayments),
        #[prost(message, tag="2")]
        DriverPayments(super::DriverPayments),
        #[prost(message, tag="3")]
        DriverFoundationPayments(super::DriverFoundationPayments),
        #[prost(message, tag="4")]
        AiTrainerPayments(super::AiTrainerPayments),
        #[prost(message, tag="5")]
        Transfers(super::Transfers),
        #[prost(message, tag="6")]
        Mints(super::Mints),
        #[prost(message, tag="7")]
        Burs(super::Burns),
        #[prost(message, tag="8")]
        TransferChecks(super::TransferCheckeds),
        #[prost(message, tag="9")]
        MintToChecks(super::MintToCheckeds),
        #[prost(message, tag="10")]
        BurnChecks(super::BurnCheckeds),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenSplittingPayments {
    #[prost(message, repeated, tag="1")]
    pub payments: ::prost::alloc::vec::Vec<TokenSplittingPayment>,
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
pub struct DriverPayments {
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<DriverPayment>,
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
pub struct DriverFoundationPayments {
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<DriverFoundationPayment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriverFoundationPayment {
    #[prost(message, optional, tag="1")]
    pub payment: ::core::option::Option<Payment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiTrainerPayments {
    #[prost(message, repeated, tag="1")]
    pub payments: ::prost::alloc::vec::Vec<AiTrainerPayment>,
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
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(double, tag="4")]
    pub amount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
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
pub struct Mints {
    #[prost(message, repeated, tag="1")]
    pub mints: ::prost::alloc::vec::Vec<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub amount: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burns {
    #[prost(message, repeated, tag="1")]
    pub burns: ::prost::alloc::vec::Vec<Burn>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burn {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub amount: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCheckeds {
    #[prost(message, repeated, tag="1")]
    pub transfer_checkeds: ::prost::alloc::vec::Vec<TransferChecked>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferChecked {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
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
pub struct MintToCheckeds {
    #[prost(message, repeated, tag="1")]
    pub mint_checkeds: ::prost::alloc::vec::Vec<MintToChecked>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToChecked {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
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
pub struct BurnCheckeds {
    #[prost(message, repeated, tag="1")]
    pub burn_checkeds: ::prost::alloc::vec::Vec<BurnChecked>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnChecked {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
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
