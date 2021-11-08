#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoBlock {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<NeoBlockHeader>,
    #[prost(message, repeated, tag = "2")]
    pub tx: ::prost::alloc::vec::Vec<NeoTransaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoBlockHeader {
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
    #[prost(uint64, tag = "7")]
    pub nonce: u64,
    #[prost(uint64, tag = "8")]
    pub primary: u64,
    #[prost(message, repeated, tag = "9")]
    pub witnesses: ::prost::alloc::vec::Vec<Witness>,
    #[prost(message, optional, tag = "10")]
    pub nextconsensus: ::core::option::Option<Address>,
    #[prost(message, optional, tag = "11")]
    pub nextblockhash: ::core::option::Option<CryptoHash>,
    #[prost(uint64, tag = "12")]
    pub confirmations: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoTransaction {
    #[prost(message, optional, tag = "1")]
    pub hash: ::core::option::Option<CryptoHash>,
    #[prost(uint64, tag = "2")]
    pub size: u64,
    #[prost(uint64, tag = "3")]
    pub version: u64,
    #[prost(uint64, tag = "4")]
    pub nonce: u64,
    #[prost(message, optional, tag = "5")]
    pub sender: ::core::option::Option<Address>,
    #[prost(string, tag = "6")]
    pub sysfee: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub netfee: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub validuntilblock: u64,
    #[prost(message, repeated, tag = "9")]
    pub signers: ::prost::alloc::vec::Vec<Signer>,
    #[prost(message, repeated, tag = "10")]
    pub attributes: ::prost::alloc::vec::Vec<TransactionAttribute>,
    #[prost(bytes = "vec", tag = "11")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "12")]
    pub witnesses: ::prost::alloc::vec::Vec<Witness>,
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
    #[prost(message, optional, tag = "3")]
    pub scope: ::core::option::Option<WitnessScope>,
    #[prost(message, repeated, tag = "4")]
    pub allowed_contracts: ::prost::alloc::vec::Vec<Address>,
    #[prost(message, repeated, tag = "5")]
    pub allowed_groups: ::prost::alloc::vec::Vec<PublicKey>,
    #[prost(uint64, tag = "6")]
    pub size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessScope {
    #[prost(oneof = "witness_scope::Scope", tags = "1, 2, 3, 4, 5")]
    pub scope: ::core::option::Option<witness_scope::Scope>,
}
/// Nested message and enum types in `WitnessScope`.
pub mod witness_scope {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        #[prost(message, tag = "1")]
        CustomContracts(super::CustomContracts),
        #[prost(message, tag = "2")]
        CalledByEntry(super::CalledByEntry),
        #[prost(message, tag = "3")]
        CustomGroups(super::CustomGroups),
        #[prost(message, tag = "4")]
        Global(super::Global),
        #[prost(message, tag = "5")]
        Null(super::Null),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomContracts {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalledByEntry {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomGroups {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Global {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Null {}
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
    #[prost(message, optional, tag = "4")]
    pub code: ::core::option::Option<OracleResponseCode>,
    #[prost(bytes = "vec", tag = "5")]
    pub result: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "6")]
    pub allow_multiple: bool,
    #[prost(uint64, tag = "7")]
    pub size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleResponseCode {
    #[prost(
        oneof = "oracle_response_code::Code",
        tags = "10, 1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub code: ::core::option::Option<oracle_response_code::Code>,
}
/// Nested message and enum types in `OracleResponseCode`.
pub mod oracle_response_code {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Code {
        #[prost(message, tag = "10")]
        Success(super::Success),
        #[prost(message, tag = "1")]
        ProtocolNotSupported(super::ProtocolNotSupported),
        #[prost(message, tag = "2")]
        ConsensusUnreachable(super::ConsensusUnreachable),
        #[prost(message, tag = "3")]
        NotFound(super::NotFound),
        #[prost(message, tag = "4")]
        Timeout(super::Timeout),
        #[prost(message, tag = "5")]
        Forbidden(super::Forbidden),
        #[prost(message, tag = "6")]
        ResponseTooLarge(super::ResponseTooLarge),
        #[prost(message, tag = "7")]
        InsufficientFunds(super::InsufficientFunds),
        #[prost(message, tag = "8")]
        ContentTypeNotSupported(super::ContentTypeNotSupported),
        #[prost(message, tag = "9")]
        Error(super::Error),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Success {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolNotSupported {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusUnreachable {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotFound {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timeout {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Forbidden {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseTooLarge {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsufficientFunds {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentTypeNotSupported {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {}
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
    #[prost(message, optional, tag = "1")]
    pub tigger: ::core::option::Option<TriggerType>,
    #[prost(message, optional, tag = "2")]
    pub vmstate: ::core::option::Option<VmState>,
    #[prost(uint64, tag = "3")]
    pub gas_consumed: u64,
    #[prost(string, tag = "4")]
    pub exception_message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub stack_item: ::prost::alloc::vec::Vec<StackItem>,
    #[prost(message, repeated, tag = "6")]
    pub notify_event_args: ::prost::alloc::vec::Vec<RpcNotifyEventArgs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerType {
    #[prost(oneof = "trigger_type::Type", tags = "6, 1, 2, 3, 4, 5")]
    pub r#type: ::core::option::Option<trigger_type::Type>,
}
/// Nested message and enum types in `TriggerType`.
pub mod trigger_type {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "6")]
        OnPersist(super::OnPersist),
        #[prost(message, tag = "1")]
        PostPersist(super::PostPersist),
        #[prost(message, tag = "2")]
        Verification(super::Verification),
        #[prost(message, tag = "3")]
        Application(super::Application),
        #[prost(message, tag = "4")]
        System(super::System),
        #[prost(message, tag = "5")]
        All(super::All),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnPersist {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostPersist {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Verification {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct System {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct All {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmState {
    #[prost(oneof = "vm_state::State", tags = "5, 1, 2, 4")]
    pub state: ::core::option::Option<vm_state::State>,
}
/// Nested message and enum types in `VMState`.
pub mod vm_state {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum State {
        #[prost(message, tag = "5")]
        None(super::None),
        #[prost(message, tag = "1")]
        Halt(super::Halt),
        #[prost(message, tag = "2")]
        Fault(super::Fault),
        #[prost(message, tag = "4")]
        Break(super::Break),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct None {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Halt {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fault {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Break {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackItem {
    #[prost(message, optional, boxed, tag = "1")]
    pub null: ::core::option::Option<::prost::alloc::boxed::Box<StackItem>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub r#false: ::core::option::Option<::prost::alloc::boxed::Box<StackItem>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub r#true: ::core::option::Option<::prost::alloc::boxed::Box<StackItem>>,
    #[prost(message, optional, tag = "4")]
    pub r#type: ::core::option::Option<StackItemType>,
    #[prost(bool, tag = "5")]
    pub is_null: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackItemType {
    #[prost(
        oneof = "stack_item_type::StackType",
        tags = "10, 1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub stack_type: ::core::option::Option<stack_item_type::StackType>,
}
/// Nested message and enum types in `StackItemType`.
pub mod stack_item_type {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StackType {
        #[prost(message, tag = "10")]
        Any(super::Any),
        #[prost(message, tag = "1")]
        Pointer(super::Pointer),
        #[prost(message, tag = "2")]
        Bool(super::Boolean),
        #[prost(message, tag = "3")]
        Int(super::Integer),
        #[prost(message, tag = "4")]
        ByteString(super::ByteString),
        #[prost(message, tag = "5")]
        Buffer(super::Buffer),
        #[prost(message, tag = "6")]
        Array(super::Array),
        #[prost(message, tag = "7")]
        Struct(super::Struct),
        #[prost(message, tag = "8")]
        Map(super::Map),
        #[prost(message, tag = "9")]
        InteropInterface(super::InteropInterface),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pointer {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Boolean {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Integer {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteString {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Buffer {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Array {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Map {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteropInterface {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcNotifyEventArgs {
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Address>,
    #[prost(string, tag = "2")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<StackItem>,
}
