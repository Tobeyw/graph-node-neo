use graph::runtime::{
    AscIndexId, AscPtr, AscType, AscValue, DeterministicHostError, IndexForAscTypeId,
};
use graph::semver::Version;
use graph_runtime_derive::AscType;
use graph_runtime_wasm::asc_abi::class::{Array, AscBigInt, AscEnum, AscString, Uint8Array};
use test_store::execute_subgraph_query;


pub(crate) type AscCryptoHash = Uint8Array;
pub(crate) type AscAccount = Uint8Array;
pub(crate) type AscPublicKey = Uint8Array;
pub(crate) type AscAddress = Uint8Array;
pub(crate) type AscBigInteger = u64;

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscNeoBlockHeader {
    pub hash: AscPtr<AscCryptoHash>,
    pub size: AscBigInteger,
    pub version:AscBigInteger,
    pub previous_block_hash: AscPtr<AscCryptoHash>,
    pub merkle_root: AscPtr<AscCryptoHash>,
    pub time:  AscBigInteger,
    pub nonce: AscBigInteger,
    pub index:AscBigInteger,
    pub primary: AscBigInteger,
    pub witnesses:  AscPtr<AscWitnessArray>,
    pub next_consensus: AscPtr<AscAddress>,
    pub next_block_hash: AscPtr<AscCryptoHash>,
    pub confirmation: AscBigInteger,   
}


impl AscIndexId for AscNeoBlockHeader {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoBlockHeader;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscNeoTransaction{
    pub hash: AscPtr<AscCryptoHash>,
    pub size:  AscBigInteger,
    pub version: AscBigInteger,
    pub nonce: AscBigInteger,
    pub sender: AscPtr<AscAddress>,
    pub sysfee: AscPtr<AscString>,
    pub netfee: AscPtr<AscString>,
    pub vailduntilblock:  AscBigInteger,
    pub signers: AscPtr<AscSignerArray>,
    pub attributes: AscPtr<AscTransactionAttributeArray>,
    pub script:AscPtr<Uint8Array>,
    pub witnesses:AscPtr<AscWitnessArray>,
    pub confirmation: AscBigInteger,
    pub blocktime:AscBigInteger,
    pub blockhash:AscPtr<AscCryptoHash>,
}

 
impl AscIndexId for AscNeoTransaction {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoTransaction;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscRpcApplicationLog{

    pub txid: AscPtr<AscCryptoHash>,
    pub blockhash: AscPtr<AscCryptoHash>,
    pub executes: AscPtr<AscExecutionArray>
}


#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscNeoBlock{
    pub header: AscPtr<AscNeoBlockHeader>,
    pub tx: AscPtr<AscTransactionArray>,  
}

impl AscIndexId for AscNeoBlock {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoBlock;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscWitness{
    pub invocation: AscPtr<Uint8Array>,
    pub verification: AscPtr<Uint8Array>, 
}

impl AscIndexId for AscWitness {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoWitness;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscSigner{
    pub max_subitems: AscBigInteger,
    pub account: AscPtr<AscAccount>,
    pub scope: AscPtr<AscWitnessScopeEnum>,
    pub allowed_contracts: AscPtr<AscAddressArray>,
    pub allow_groups: AscPtr<AscPublicKeyArray>,
    pub size: AscBigInteger, 
}

impl AscIndexId for AscSigner {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoSigner;
}

#[repr(u32)]
#[derive(AscType, Copy, Clone)]
pub(crate) enum AscWitnessScope {
    CustomContracts,
    CalledByEntry,
    CustomGroups,
    Global,
    Null,   
}

impl AscValue for AscWitnessScope {}

impl Default for AscWitnessScope {
    fn default() -> Self {
        Self::CalledByEntry
    }
}
pub struct AscWitnessScopeEnum(pub(crate) AscEnum<AscWitnessScope>);

impl AscType for AscWitnessScopeEnum {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(AscEnum::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscWitnessScopeEnum {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoEnumWitnessScope;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscCustomContracts {}

impl AscIndexId for AscCustomContracts {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoCustomContracts;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscCalledByEntry {}

impl AscIndexId for AscCalledByEntry {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoCalledByEntry;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscCustomGroups {}

impl AscIndexId for AscCustomGroups {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoCustomGroups;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscNull {}

impl AscIndexId for AscNull {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoNull;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscGlobal {}

impl AscIndexId for AscGlobal {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoGlobal;
}



#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscTransactionAttribute{
    pub atype: AscPtr<AscTransactionAttributeTypeEnum>,
    pub allow_multiple:  bool,
    pub size: AscBigInteger,  
}

impl AscIndexId for AscTransactionAttribute {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoTransactionAttribute;
}

#[repr(u32)]
#[derive(AscType, Copy, Clone)]
pub(crate) enum AscTransactionAttributeType {
    HighPriority,
    OracleResponse,
}

impl AscValue for AscTransactionAttributeType {}

impl Default for AscTransactionAttributeType {
    fn default() -> Self {
        Self::HighPriority
    }
}
#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscHighPriority {
    pub allow_multiple: bool,
    pub transaction_attribute_type :AscPtr<AscTransactionAttributeTypeEnum>

}

impl AscIndexId for AscHighPriority {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoHighPriority;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscOracleResponse {
    pub max_result_size :u64,
    pub fixed_script :AscPtr<Uint8Array>,
    pub id :u64,
    pub code :AscPtr<AscOracleResponseCodeEnum>,
    pub result:AscPtr<Uint8Array>,
    pub allow_multiple:bool,
    pub size : u64
}

impl AscIndexId for AscOracleResponse {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoOracleResponse;
}


#[repr(u32)]
#[derive(AscType, Copy, Clone)]
pub(crate) enum AscOracleResponseCode {
    Success,
    ProtocolNotSupported,
    ConsensusUnreachable,
    NotFound,
    Timeout,
    Forbidden,
    ResponseTooLarge,
    InsufficientFunds,
    ContentTypeNotSupported,
    Error,
}

impl AscValue for AscOracleResponseCode {}

impl Default for AscOracleResponseCode {
    fn default() -> Self {
        Self::Error
    }
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscSuccess {}

impl AscIndexId for AscSuccess{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoSuccess;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscProtocolNotSupported {}

impl AscIndexId for AscProtocolNotSupported{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoProtocolNotSupported;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscConsensusUnreachable {}

impl AscIndexId for AscConsensusUnreachable{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoConsensusUnreachable;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscNotFound {}

impl AscIndexId for AscNotFound{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoNotFound;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscTimeout {}

impl AscIndexId for AscTimeout{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoTimeout;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscForbidden {}

impl AscIndexId for AscForbidden{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoForbidden;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscResponseTooLarge {}

impl AscIndexId for AscResponseTooLarge{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoResponseTooLarge;
}


#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscInsufficientFunds {}

impl AscIndexId for AscInsufficientFunds{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoInsufficientFunds;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscContentTypeNotSupported {}

impl AscIndexId for AscContentTypeNotSupported{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoContentTypeNotSupported;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscError {}

impl AscIndexId for AscError{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoError;
}




pub struct AscOracleResponseCodeEnum(pub(crate) AscEnum<AscOracleResponseCode>);

impl AscType for AscOracleResponseCodeEnum {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(AscEnum::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscOracleResponseCodeEnum {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoEnumOracleResponseCode;
}

pub struct AscTransactionAttributeTypeEnum(pub(crate) AscEnum<AscTransactionAttributeType>);

impl AscType for AscTransactionAttributeTypeEnum {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(AscEnum::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscTransactionAttributeTypeEnum {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoEnumTransactionAttributeType;
}

#[repr(u32)]
#[derive(AscType, Copy, Clone)]
pub(crate) enum AscTriggerType {
    OnPersist,
    PostPersist,
    Verification,
    Application,
    System,
    All,
}

impl AscValue for AscTriggerType {}

impl Default for AscTriggerType {
    fn default() -> Self {
        Self::Application
    }
}
pub struct AscTriggerTypeEnum(pub(crate) AscEnum<AscTriggerType>);

impl AscType for AscTriggerTypeEnum {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(AscEnum::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscTriggerTypeEnum {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoEnumTriggerType;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscOnPersist {}

impl AscIndexId for AscOnPersist {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoOnPersist;
}
#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscPostPersist {}

impl AscIndexId for AscPostPersist {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoPostPersist;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscVerification {}

impl AscIndexId for AscVerification{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoVerification;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscApplication {}

impl AscIndexId for AscApplication{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoApplication;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscSystem {}

impl AscIndexId for AscSystem{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoSystem;
}
#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscAll {}

impl AscIndexId for AscAll{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoAll;
}


#[repr(u32)]
#[derive(AscType, Copy, Clone)]
pub(crate) enum AscVMState {
    None,
    HALT,
    FAULT,
    BREAK,
}

impl AscValue for AscVMState {}

impl Default for AscVMState {
    fn default() -> Self {
        Self::None
    }
} 
pub struct AscVMStateEnum(pub(crate) AscEnum<AscVMState>);

impl AscType for AscVMStateEnum {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(AscEnum::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscVMStateEnum {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoEnumVMState;
}


#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscNONE {}

impl AscIndexId for AscNONE{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoNONE;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscHALT {}

impl AscIndexId for AscHALT{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoHALT;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscFAULT {}

impl AscIndexId for AscFAULT{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoFAULT;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscBREAK {}

impl AscIndexId for AscBREAK{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoBREAK;
}

//=====

#[repr(u32)]
#[derive(AscType, Copy, Clone)]
pub(crate) enum AscStackItemType {
    Any,
    Pointer,
    Boolean,
    Integer,
    ByteString,
    Buffer,
    Array,
    Struct,
    Map,
    InteropInterface,
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscAny {}

impl AscIndexId for AscAny{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoAny;
}


#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscPointer {}

impl AscIndexId for AscPointer{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoPointer;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscBoolean {}

impl AscIndexId for AscBoolean{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoBoolean;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscInteger {}

impl AscIndexId for AscInteger{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoInteger;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscByteString {}

impl AscIndexId for AscByteString{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoByteString;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscBuffer {}

impl AscIndexId for AscBuffer{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoBuffer;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscArray {}

impl AscIndexId for AscArray{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArray;
}

 

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscStruct {}

impl AscIndexId for AscStruct{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoStruct;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscMap {}

impl AscIndexId for AscMap{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoMap;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscInteropInterface {}

impl AscIndexId for AscInteropInterface{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoInteropInterface;
}


impl AscValue for AscStackItemType {}

impl Default for AscStackItemType {
    fn default() -> Self {
        Self::Any
    }
}
pub struct AscStackItemTypeEnum(pub(crate) AscEnum<AscStackItemType>);

impl AscType for AscStackItemTypeEnum {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(AscEnum::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscStackItemTypeEnum {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoEnumStackItem;
} 

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscExecution{
    pub trigger: AscPtr<AscTriggerTypeEnum>,
    pub vmstate: AscPtr<AscVMStateEnum>,
    pub gas_consumsed: AscBigInteger,  
    pub exception_message: AscPtr<AscString>, 
    pub stack_item : AscPtr<AscStackItemArray>,
    pub notifify_eventsArgs:AscPtr<AscRpcNotifyEventArgsArray>,
}

impl AscIndexId for AscExecution {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoExecution;
}

pub struct AscExecutionArray(pub(crate) Array<AscPtr<AscExecution>>);

impl AscType for AscExecutionArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}



#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscStackItem{
    pub stack_null: AscPtr<AscStackItem>,
    pub stack_false: AscPtr<AscStackItem>,
    pub stack_true: AscPtr<AscStackItem>,  
    pub stack_type: AscPtr<AscStackItemTypeEnum>, 
    pub is_null : bool,
}

impl AscIndexId for AscStackItem {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoStackItem;
}

#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscRpcNotifyEventArgs{
    pub contract: AscPtr<AscAddress>,
    pub event_name: AscString,
    pub state: AscPtr<AscStackItem>,  
    
}

impl AscIndexId for AscRpcNotifyEventArgs{
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoRpcNotifyEventArgs;
}

// Array


pub struct AscCryptoHashArray(pub(crate) Array<AscPtr<AscCryptoHash>>);

impl AscType for AscCryptoHashArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscCryptoHashArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayCryptoHash;
}

pub struct AscAccountArray(pub(crate) Array<AscPtr<AscAccount>>);

impl AscType for AscAccountArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscAccountArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayAccount;
}

pub struct AscAddressArray(pub(crate) Array<AscPtr<AscAddress>>);

impl AscType for AscAddressArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscAddressArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayAddress;
}

 


pub struct AscRpcNotifyEventArgsArray(pub(crate) Array<AscPtr<AscRpcNotifyEventArgs>>);

impl AscType for AscRpcNotifyEventArgsArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscRpcNotifyEventArgsArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayRpcNotifyEventArgs;
}
pub struct AscStackItemArray(pub(crate) Array<AscPtr<AscStackItem>>);

impl AscType for AscStackItemArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscStackItemArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayStackItem;
}
//
pub struct AscTransactionArray(pub(crate) Array<AscPtr<AscNeoTransaction>>);

impl AscType for AscTransactionArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscTransactionArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayTransaction;
}

pub struct AscWitnessArray(pub(crate) Array<AscPtr<AscWitness>>);

impl AscType for AscWitnessArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscWitnessArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayWitness;
}

pub struct AscTransactionAttributeArray(pub(crate) Array<AscPtr<AscTransactionAttribute>>);

impl AscType for AscTransactionAttributeArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscTransactionAttributeArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayTransactionAttribute;
}

pub struct AscPublicKeyArray(pub(crate) Array<AscPtr<AscPublicKey>>);

impl AscType for AscPublicKeyArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscPublicKeyArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArrayPublicKey;
}


pub struct AscSignerArray(pub(crate) Array<AscPtr<AscSigner>>);

impl AscType for AscSignerArray {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(
        asc_obj: &[u8],
        api_version: &Version,
    ) -> Result<Self, DeterministicHostError> {
        Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
    }
}

impl AscIndexId for AscSignerArray {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoArraySigner;
}






