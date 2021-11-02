#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<BlockHeader>,
    #[prost(message, repeated, tag = "2")]
    pub tx: ::prost::alloc::vec::Vec<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(message, optional, tag = "1")]
    pub hash: ::core::option::Option<CryptoHash>,
    #[prost(uint64, tag = "2")]
    pub size: u64,
    #[prost(uint64, tag = "3")]
    pub version: u64,
    #[prost(message, optional, tag = "4")]
    pub previous_block_hash: ::core::option::Option<CryptoHash>,
    #[prost(message, optional, tag = "5")]
    pub merkrle_root: ::core::option::Option<CryptoHash>,
    #[prost(uint64, tag = "6")]
    pub time: u64,
    ///
    #[prost(uint64, tag = "7")]
    pub nounce: u64,
    #[prost(uint64, tag = "8")]
    pub primary: u64,
    #[prost(message, repeated, tag = "9")]
    pub witness: ::prost::alloc::vec::Vec<Witness>,
    #[prost(message, optional, tag = "10")]
    pub nextconsensus: ::core::option::Option<Address>,
    #[prost(message, optional, tag = "11")]
    pub nextblockhash: ::core::option::Option<CryptoHash>,
    #[prost(uint64, tag = "12")]
    pub confirmations: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag = "1")]
    pub hash: ::core::option::Option<CryptoHash>,
    #[prost(uint64, tag = "2")]
    pub size: u64,
    #[prost(uint64, tag = "3")]
    pub version: u64,
    ///
    #[prost(uint64, tag = "4")]
    pub nounce: u64,
    #[prost(message, optional, tag = "5")]
    pub sender: ::core::option::Option<Address>,
    #[prost(uint64, tag = "6")]
    pub sysfee: u64,
    #[prost(uint64, tag = "7")]
    pub netfee: u64,
    #[prost(uint64, tag = "8")]
    pub validuntilblock: u64,
    #[prost(message, repeated, tag = "9")]
    pub signers: ::prost::alloc::vec::Vec<Signer>,
    #[prost(message, repeated, tag = "10")]
    pub attributes: ::prost::alloc::vec::Vec<TransactionAttribute>,
    #[prost(bytes = "vec", tag = "11")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "12")]
    pub witness: ::prost::alloc::vec::Vec<Witness>,
    #[prost(uint64, tag = "13")]
    pub confirmations: u64,
    #[prost(uint64, tag = "14")]
    pub blocktime: u64,
    #[prost(message, optional, tag = "15")]
    pub blockhash: ::core::option::Option<CryptoHash>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Witness {
    #[prost(bytes = "vec", tag = "1")]
    pub invocation: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub verification: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signer {
    #[prost(uint64, tag = "1")]
    pub max_subitems: u64,
    #[prost(message, optional, tag = "2")]
    pub account: ::core::option::Option<Account>,
    #[prost(enumeration = "WitnessScope", tag = "3")]
    pub scope: i32,
    #[prost(message, repeated, tag = "4")]
    pub allowed_contracts: ::prost::alloc::vec::Vec<Address>,
    #[prost(message, repeated, tag = "5")]
    pub allowed_groups: ::prost::alloc::vec::Vec<PublicKey>,
    #[prost(uint64, tag = "6")]
    pub size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionAttribute {
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<TransactionAttributeType>,
    #[prost(bool, tag = "2")]
    pub allow_multiple: bool,
    #[prost(uint64, tag = "3")]
    pub size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionAttributeType {
    #[prost(oneof = "transaction_attribute_type::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<transaction_attribute_type::Type>,
}
/// Nested message and enum types in `TransactionAttributeType`.
pub mod transaction_attribute_type {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        HighPriority(::prost::alloc::boxed::Box<super::HighPriority>),
        #[prost(message, tag = "2")]
        OracleResponse(super::OracleResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighPriority {
    #[prost(bool, tag = "1")]
    pub allow_multiple: bool,
    #[prost(message, optional, boxed, tag = "2")]
    pub transaction_attribute_type:
        ::core::option::Option<::prost::alloc::boxed::Box<TransactionAttributeType>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleResponse {
    #[prost(uint64, tag = "1")]
    pub max_result_size: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub fixed_script: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub id: u64,
    #[prost(enumeration = "OracleResponseCode", tag = "4")]
    pub code: i32,
    #[prost(bytes = "vec", tag = "5")]
    pub result: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "6")]
    pub allow_multiple: bool,
    #[prost(uint64, tag = "7")]
    pub size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoHash {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Execution {
    #[prost(enumeration = "TriggerType", tag = "1")]
    pub tigger: i32,
    #[prost(enumeration = "VmState", tag = "2")]
    pub vmstate: i32,
    #[prost(uint64, tag = "3")]
    pub gasc_consued: u64,
    #[prost(string, tag = "4")]
    pub exception_message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub stack_item: ::prost::alloc::vec::Vec<StackItem>,
    #[prost(message, repeated, tag = "6")]
    pub notify_event_args: ::prost::alloc::vec::Vec<RpcNotifyEventArgs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackItem {
    #[prost(message, optional, boxed, tag = "1")]
    pub null: ::core::option::Option<::prost::alloc::boxed::Box<StackItem>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub r#false: ::core::option::Option<::prost::alloc::boxed::Box<StackItem>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub r#true: ::core::option::Option<::prost::alloc::boxed::Box<StackItem>>,
    #[prost(enumeration = "StackItemType", tag = "4")]
    pub r#type: i32,
    #[prost(bool, tag = "5")]
    pub is_null: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcNotifyEventArgs {
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Address>,
    #[prost(string, tag = "2")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<StackItem>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WitnessScope {
    CustomContracts = 0,
    CalledByEntry = 1,
    CustomGroups = 2,
    Global = 3,
    Null = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OracleResponseCode {
    Success = 0,
    ProtocolNotSupported = 1,
    ConsensusUnreachable = 2,
    NotFound = 3,
    Timeout = 4,
    Forbidden = 5,
    ResponseTooLarge = 6,
    InsufficientFunds = 7,
    ContentTypeNotSupported = 8,
    Error = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    OnPersist = 0,
    PostPersist = 1,
    Verification = 2,
    Application = 3,
    System = 4,
    All = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VmState {
    None = 0,
    Halt = 1,
    Fault = 2,
    Break = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StackItemType {
    Any = 0,
    Pointer = 1,
    Boolean = 2,
    Integer = 3,
    ByteString = 4,
    Buffer = 5,
    Array = 6,
    Struct = 7,
    Map = 8,
    InteropInterface = 9,
}
