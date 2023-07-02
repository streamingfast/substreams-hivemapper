// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenSplittingPayments {
    #[prost(message, repeated, tag="1")]
    pub payments: ::prost::alloc::vec::Vec<TokenSplittingPayment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenSplittingPayment {
    #[prost(message, optional, tag="1")]
    pub manager: ::core::option::Option<Payment>,
    #[prost(message, optional, tag="2")]
    pub driver: ::core::option::Option<Payment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriverPayments {
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<DriverPayment>,
}
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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriverFoundationPayments {
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<DriverFoundationPayment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriverFoundationPayment {
    #[prost(message, optional, tag="1")]
    pub payment: ::core::option::Option<Payment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiTrainerPayments {
    #[prost(message, repeated, tag="1")]
    pub payments: ::prost::alloc::vec::Vec<AiTrainerPayment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiTrainerPayment {
    #[prost(message, optional, tag="1")]
    pub payment: ::core::option::Option<Payment>,
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
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
/// Encoded file descriptor set for the `hivemapper.types.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa3, 0x17, 0x0a, 0x1e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2f,
    0x76, 0x31, 0x2f, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x13, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x22, 0x60, 0x0a, 0x16, 0x54, 0x6f, 0x6b, 0x65,
    0x6e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x12, 0x46, 0x0a, 0x08, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65,
    0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e,
    0x53, 0x70, 0x6c, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74,
    0x52, 0x08, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x22, 0x85, 0x01, 0x0a, 0x15, 0x54,
    0x6f, 0x6b, 0x65, 0x6e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x61, 0x79,
    0x6d, 0x65, 0x6e, 0x74, 0x12, 0x36, 0x0a, 0x07, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70,
    0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x61, 0x79, 0x6d,
    0x65, 0x6e, 0x74, 0x52, 0x07, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x12, 0x34, 0x0a, 0x06,
    0x64, 0x72, 0x69, 0x76, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68,
    0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e,
    0x76, 0x31, 0x2e, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x06, 0x64, 0x72, 0x69, 0x76,
    0x65, 0x72, 0x22, 0x4e, 0x0a, 0x0e, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x50, 0x61, 0x79, 0x6d,
    0x65, 0x6e, 0x74, 0x73, 0x12, 0x3c, 0x0a, 0x07, 0x72, 0x65, 0x77, 0x61, 0x72, 0x64, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70,
    0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x72, 0x69, 0x76,
    0x65, 0x72, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x07, 0x72, 0x65, 0x77, 0x61, 0x72,
    0x64, 0x73, 0x22, 0xc0, 0x01, 0x0a, 0x0d, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x50, 0x61, 0x79,
    0x6d, 0x65, 0x6e, 0x74, 0x12, 0x36, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70,
    0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x61, 0x79, 0x6d,
    0x65, 0x6e, 0x74, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x42, 0x0a, 0x04,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2e, 0x2e, 0x68, 0x69, 0x76,
    0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31,
    0x2e, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x44,
    0x52, 0x49, 0x56, 0x45, 0x52, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x22, 0x33, 0x0a, 0x0b, 0x44, 0x52, 0x49, 0x56, 0x45, 0x52, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x12,
    0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x52, 0x45,
    0x47, 0x55, 0x4c, 0x41, 0x52, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x4e, 0x4f, 0x5f, 0x53, 0x50,
    0x4c, 0x49, 0x54, 0x10, 0x02, 0x22, 0x62, 0x0a, 0x18, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x46,
    0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74,
    0x73, 0x12, 0x46, 0x0a, 0x07, 0x72, 0x65, 0x77, 0x61, 0x72, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x46,
    0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74,
    0x52, 0x07, 0x72, 0x65, 0x77, 0x61, 0x72, 0x64, 0x73, 0x22, 0x51, 0x0a, 0x17, 0x44, 0x72, 0x69,
    0x76, 0x65, 0x72, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x79,
    0x6d, 0x65, 0x6e, 0x74, 0x12, 0x36, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70,
    0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x61, 0x79, 0x6d,
    0x65, 0x6e, 0x74, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x22, 0x56, 0x0a, 0x11,
    0x41, 0x69, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74,
    0x73, 0x12, 0x41, 0x0a, 0x08, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x69, 0x54, 0x72, 0x61, 0x69,
    0x6e, 0x65, 0x72, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x08, 0x70, 0x61, 0x79, 0x6d,
    0x65, 0x6e, 0x74, 0x73, 0x22, 0x4a, 0x0a, 0x10, 0x41, 0x69, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x36, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6d,
    0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x69, 0x76, 0x65,
    0x6d, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e,
    0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74,
    0x22, 0x80, 0x01, 0x0a, 0x07, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x1c, 0x0a, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52,
    0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x25, 0x0a, 0x0e, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0d, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49,
    0x64, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x61,
    0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x06, 0x61, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x22, 0x48, 0x0a, 0x09, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x73,
    0x12, 0x3b, 0x0a, 0x09, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x68, 0x69, 0x76, 0x65, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x72,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x65, 0x72, 0x52, 0x09, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x73, 0x22, 0x7b, 0x0a,
    0x08, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x12, 0x15, 0x0a, 0x06, 0x74, 0x72, 0x78,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x72, 0x78, 0x49, 0x64,
    0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x12,
    0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x66, 0x72,
    0x6f, 0x6d, 0x12, 0x0e, 0x0a, 0x02, 0x74, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02,
    0x74, 0x6f, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x4a, 0xc1, 0x0d, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x3d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x05, 0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x21, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x05, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00,
    0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x09, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x0a, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x09, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a,
    0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0a, 0x02, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x0a, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x0d, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x0d, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0e, 0x0b, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x11, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x11, 0x08,
    0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x12, 0x02, 0x16, 0x03, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x12, 0x07, 0x12, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x13, 0x0c, 0x0d, 0x0a, 0x2c, 0x0a, 0x06,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x04, 0x10, 0x22, 0x1d, 0x20, 0x53, 0x6f,
    0x6d, 0x65, 0x6f, 0x6e, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x70, 0x61, 0x72, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x66, 0x6c, 0x65, 0x65, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x14, 0x0e, 0x0f, 0x0a, 0x50, 0x0a, 0x06, 0x04, 0x03,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x15, 0x04, 0x11, 0x22, 0x41, 0x20, 0x57, 0x65, 0x20, 0x63,
    0x61, 0x6e, 0x27, 0x74, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x27,
    0x73, 0x20, 0x61, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x61,
    0x20, 0x66, 0x6c, 0x65, 0x65, 0x74, 0x20, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x61, 0x74,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x15, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x18, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x18, 0x0a, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x18, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x19, 0x02, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x19, 0x02, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x1c, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1c,
    0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x0b, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x23, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1d, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x20,
    0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x20, 0x08, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x0a, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x21, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x24, 0x00,
    0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x24, 0x08, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x25, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x25, 0x1c, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x25, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x28, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x29, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x29, 0x0a, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29,
    0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x2c, 0x00, 0x31, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x2d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2d, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x14,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x02, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02,
    0x12, 0x03, 0x2f, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x2f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x09,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x13, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x30, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x30, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x30, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x33, 0x00,
    0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x33, 0x08, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x34, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x34, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x34, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x34, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x37, 0x00, 0x3d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x38, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x38, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38,
    0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x39, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x02, 0x12, 0x03, 0x3a, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x3a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a,
    0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x10, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x02, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3b, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x3b, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x04, 0x12,
    0x03, 0x3c, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x03, 0x3c,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3c, 0x09, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3c, 0x12, 0x13, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)