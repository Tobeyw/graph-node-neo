use graph::runtime::{
    AscIndexId, AscPtr, AscType, AscValue, DeterministicHostError, IndexForAscTypeId,
};
use graph::semver::Version;
use graph_runtime_derive::AscType;
use graph_runtime_wasm::asc_abi::class::{Array, AscEnum, AscString, Uint8Array};


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
    pub sysfee: AscPtr<AscCryptoHash>,
    pub netfee: AscPtr<AscCryptoHash>,
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
pub(crate) struct AscNeoBlock{
    pub header: AscPtr<AscNeoBlockHeader>,
    pub tx: AscPtr<AscNeoTransaction>,  
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
    pub scope: AscPtr<AscWitnessScope>,
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


#[repr(C)]
#[derive(AscType)]
pub(crate) struct AscTransactionAttribute{
    pub atype: AscPtr<AscTransactionAttributeTypeEnum>,
    pub allow_multiple: AscPtr<bool>,
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
pub(crate) struct AscExcution{
    pub trigger: AscPtr<AscTriggerTypeEnum>,
    pub vmstate: AscPtr<AscVMStateEnum>,
    pub gas_consumsed: AscBigInteger,  
    pub exception_message: AscPtr<AscString>, 
    pub stack_item : AscPtr<AscStackItemArray>,
    pub notifify_eventsArgs:AscPtr<AscRpcNotifyEventArgsArray>,
}

impl AscIndexId for AscExcution {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::NeoExcution;
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






