use crate::codec; 
use graph::runtime::{asc_new, AscHeap, DeterministicHostError, ToAscObj};
use graph_runtime_wasm::asc_abi::class::{Array, AscAddress, AscEnum, EnumPayload};

pub(crate) use super::generated::*;

impl ToAscObj<AscNeoBlock> for codec::NeoBlock{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscNeoBlock, DeterministicHostError> {  
         Ok(AscNeoBlock {
            header: asc_new(heap, self.header.as_ref().unwrap())?,
            tx: asc_new(heap, &self.tx )?,           
        })
    }
} 

impl ToAscObj<AscNeoBlockHeader> for codec::NeoBlockHeader {
    fn to_asc_obj<H: AscHeap + ?Sized>(
         &self,
        heap: &mut H,
    ) -> Result<AscNeoBlockHeader, DeterministicHostError> {
        Ok(AscNeoBlockHeader {
            hash:asc_new(heap,&self.hash.as_ref().unwrap())?,
            size: self.size,
            version: self.version,
            previous_block_hash: asc_new(heap,self.previous_block_hash.as_ref().unwrap())?,
            merkle_root: asc_new(heap,self.merkrle_root.as_ref().unwrap())?,
            time: self.time ,
            nonce: self.nonce ,
            index : self.index,
            primary: self.primary ,
            witnesses: asc_new(heap,&self.witnesses)?,//
            next_consensus: asc_new(heap,self.nextconsensus.as_ref().unwrap())?,
            next_block_hash:  asc_new(heap,self.nextblockhash.as_ref().unwrap())?,
            confirmation:  self.confirmations ,
        })
    }
}
 
impl ToAscObj<AscWitness> for codec::Witness {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscWitness, DeterministicHostError> {
        Ok(AscWitness {
            invocation:asc_new(heap,self.invocation.as_slice())?,      
            verification: asc_new(heap,self.invocation.as_slice())?,          
        })
    }
}

impl ToAscObj<AscWitnessArray> for Vec<codec::Witness> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscWitnessArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscWitnessArray(Array::new(&*content, heap)?))
    }
}

 

impl ToAscObj<AscNeoTransaction> for codec::NeoTransaction {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscNeoTransaction, DeterministicHostError> {
        Ok(AscNeoTransaction {
            hash:asc_new(heap,&self.hash.as_ref().unwrap())?,
            size: self.size,
            version: self.version,
            nonce: self.nonce,
            sender: asc_new(heap,self.sender.as_ref().unwrap())?,
            sysfee: asc_new(heap,&self.sysfee)?,
            netfee: asc_new(heap,&self.netfee)?,
            vailduntilblock: self.validuntilblock ,
            signers: asc_new(heap,&self.signers)?,//
            attributes: asc_new(heap,&self.attributes)?,
            script:  asc_new(heap,self.script.as_slice())?,
            witnesses: asc_new(heap,&self.witnesses)?,     
            confirmation: self.confirmations ,    
            blocktime: self.blocktime ,       
            blockhash: asc_new(heap,self.blockhash.as_ref().unwrap())?,   
        })
    }
}

impl ToAscObj<AscSigner> for codec::Signer {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscSigner, DeterministicHostError> {      
        Ok(AscSigner {
            max_subitems:self.max_subitems,
            account: asc_new(heap,self.account.as_ref().unwrap())?,
            scope: asc_new(heap,self.scope.as_ref().unwrap())?,    
            allowed_contracts:asc_new(heap,&self.allowed_contracts)?,
            allow_groups: asc_new(heap,&self.allowed_groups)?,
            size: self.size,        
        })
    }
}

impl ToAscObj<AscSignerArray> for Vec<codec::Signer> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscSignerArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscSignerArray(Array::new(&*content, heap)?))
    }
}

impl ToAscObj<AscTransactionAttribute> for codec::TransactionAttribute {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscTransactionAttribute, DeterministicHostError> {      
        Ok(AscTransactionAttribute {
            atype: asc_new(heap,self.r#type.as_ref().unwrap())?,//
            allow_multiple: self.allow_multiple,
            size: self.size,    
              
        })
    }
}

impl ToAscObj<AscTransactionAttributeTypeEnum> for codec::TransactionAttributeType {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscTransactionAttributeTypeEnum, DeterministicHostError> {
        let (kind, payload) = match self.r#type.as_ref().unwrap() {
            codec::transaction_attribute_type::Type::HighPriority(action) => (
                AscTransactionAttributeType::HighPriority,
                asc_new(heap, action.as_mut())?.to_payload(),
            ),
            codec::transaction_attribute_type::Type::OracleResponse(action) => (
                AscTransactionAttributeType::OracleResponse,
                asc_new(heap, action)?.to_payload(),
            ),            
        };

        Ok(AscTransactionAttributeTypeEnum(AscEnum {
            kind,
            _padding: 0,
            payload: EnumPayload(payload),
        }))
    }
}

impl ToAscObj<AscTransactionAttributeArray> for Vec<codec::TransactionAttribute> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscTransactionAttributeArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscTransactionAttributeArray(Array::new(&*content, heap)?))
    }
}

impl ToAscObj<AscWitnessScopeEnum> for codec::WitnessScope {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscWitnessScopeEnum, DeterministicHostError> {
        let (kind, payload) = match self.scope.as_ref().unwrap() {
            codec::witness_scope::Scope::CustomContracts(action) => (
                AscWitnessScope::CustomContracts,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::witness_scope::Scope::CalledByEntry(action) => (
                AscWitnessScope::CalledByEntry,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::witness_scope::Scope::CustomGroups(action) => (
                AscWitnessScope::CustomGroups,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::witness_scope::Scope::Global(action) => {
                (AscWitnessScope::Global,
                asc_new(heap, action)?.to_payload())
            }
            codec::witness_scope::Scope::Null(action) => (
                AscWitnessScope::Null,
                asc_new(heap, action)?.to_payload(),
            ),
           
           
        };

        Ok(AscWitnessScopeEnum(AscEnum {
            kind,
            _padding: 0,
            payload: EnumPayload(payload),
        }))
    }
}

impl ToAscObj<AscOracleResponseCodeEnum> for codec::OracleResponseCode {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscOracleResponseCodeEnum, DeterministicHostError> {
        let (kind, payload) = match self.code.as_ref().unwrap() {
            codec::oracle_response_code::Code::Success(action) => (
                AscOracleResponseCode::Success,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::ProtocolNotSupported(action) => (
                AscOracleResponseCode::ProtocolNotSupported,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::ConsensusUnreachable(action) => (
                AscOracleResponseCode::ConsensusUnreachable,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::NotFound(action) => (
                AscOracleResponseCode::NotFound,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::Timeout(action) => (
                AscOracleResponseCode::Timeout,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::Forbidden(action) => (
                AscOracleResponseCode::Forbidden,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::ResponseTooLarge(action) => (
                AscOracleResponseCode::ResponseTooLarge,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::InsufficientFunds(action) => (
                AscOracleResponseCode::InsufficientFunds,
                asc_new(heap, action)?.to_payload(),
            ), codec::oracle_response_code::Code::ContentTypeNotSupported(action) => (
                AscOracleResponseCode::ContentTypeNotSupported,
                asc_new(heap, action)?.to_payload(),
            ),
            codec::oracle_response_code::Code::Error(action) => (
                AscOracleResponseCode::Error,
                asc_new(heap, action)?.to_payload(),
            )
           
           
        };

        Ok(AscOracleResponseCodeEnum(AscEnum {
            kind,
            _padding: 0,
            payload: EnumPayload(payload),
        }))
    }
}


//自定义枚举类型
 

impl ToAscObj<AscHighPriority> for codec::HighPriority {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscHighPriority, DeterministicHostError> {      
        Ok(AscHighPriority {
            allow_multiple:  self.allow_multiple,          
            transaction_attribute_type: asc_new(heap, self.transaction_attribute_type.as_ref().unwrap().as_mut())?,   
              
        })
    }
}



impl ToAscObj<AscOracleResponse> for codec::OracleResponse {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscOracleResponse, DeterministicHostError> {
       // let code = self.code.as_ref().unwrap();
        Ok(AscOracleResponse {
            max_result_size:self.max_result_size,
            fixed_script:asc_new(heap, self.fixed_script.as_slice())?,
            id:self.id,
            code:asc_new(heap, self.code.as_ref().unwrap())?,
            result:asc_new(heap, self.result.as_slice())?,
            allow_multiple:self.allow_multiple,
            size:self.size,

        })
    }
}

//WitnessScope
impl ToAscObj<AscCustomContracts> for codec::CustomContracts {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscCustomContracts, DeterministicHostError> {
        Ok(AscCustomContracts {})
    }
}

impl ToAscObj<AscCalledByEntry> for codec::CalledByEntry {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscCalledByEntry, DeterministicHostError> {
        Ok(AscCalledByEntry {})
    }
}
impl ToAscObj<AscCustomGroups> for codec::CustomGroups {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscCustomGroups, DeterministicHostError> {
        Ok(AscCustomGroups {})
    }
}
impl ToAscObj<AscGlobal> for codec::Global {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscGlobal, DeterministicHostError> {
        Ok(AscGlobal {})
    }
}
impl ToAscObj<AscNull> for codec::Null {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscNull, DeterministicHostError> {
        Ok(AscNull{})
    }
}

//OracleResponseCode
impl ToAscObj<AscSuccess> for codec::Success {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscSuccess, DeterministicHostError> {
        Ok(AscSuccess{})
    }
}
impl ToAscObj<AscProtocolNotSupported> for codec::ProtocolNotSupported {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscProtocolNotSupported, DeterministicHostError> {
        Ok(AscProtocolNotSupported{})
    }
}
impl ToAscObj<AscConsensusUnreachable> for codec::ConsensusUnreachable {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscConsensusUnreachable, DeterministicHostError> {
        Ok(AscConsensusUnreachable{})
    }
}
impl ToAscObj<AscNotFound> for codec::NotFound {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscNotFound, DeterministicHostError> {
        Ok(AscNotFound{})
    }
}
impl ToAscObj<AscTimeout> for codec::Timeout {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscTimeout, DeterministicHostError> {
        Ok(AscTimeout{})
    }
}
impl ToAscObj<AscForbidden> for codec::Forbidden {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscForbidden, DeterministicHostError> {
        Ok(AscForbidden{})
    }
}
impl ToAscObj<AscResponseTooLarge> for codec::ResponseTooLarge {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscResponseTooLarge, DeterministicHostError> {
        Ok(AscResponseTooLarge{})
    }
}
impl ToAscObj<AscInsufficientFunds> for codec::InsufficientFunds {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscInsufficientFunds, DeterministicHostError> {
        Ok(AscInsufficientFunds{})
    }
}
impl ToAscObj<AscContentTypeNotSupported> for codec::ContentTypeNotSupported {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscContentTypeNotSupported, DeterministicHostError> {
        Ok(AscContentTypeNotSupported{})
    }
}
impl ToAscObj<AscError> for codec::Error {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
    ) -> Result<AscError, DeterministicHostError> {
        Ok(AscError{})
    }
}


//自定义基本类型
impl ToAscObj<AscCryptoHash> for codec::CryptoHash{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscCryptoHash, DeterministicHostError> {
       
        self.hash.to_asc_obj(heap)
    }
}

impl ToAscObj<AscCryptoHashArray> for Vec<codec::CryptoHash> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscCryptoHashArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscCryptoHashArray(Array::new(&*content, heap)?))
    }
}

impl ToAscObj<AscAccount> for codec::Account{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscAccount, DeterministicHostError> {
       
        self.hash.to_asc_obj(heap)
    }
}
 
impl ToAscObj<AscAccountArray> for Vec<codec::Account> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscAccountArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscAccountArray(Array::new(&*content, heap)?))
    }
}

impl ToAscObj<AscAddress> for codec::Address{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscAddress, DeterministicHostError> {
       
        self.address.to_asc_obj(heap)
    }
}

impl ToAscObj<AscAddressArray> for Vec<codec::Address> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscAddressArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscAddressArray(Array::new(&*content, heap)?))
    }
}

impl ToAscObj<AscPublicKey> for codec::PublicKey{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscPublicKey, DeterministicHostError> {
       
        self.bytes.to_asc_obj(heap)
    }
}

impl ToAscObj<AscPublicKeyArray> for Vec<codec::PublicKey> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscPublicKeyArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscPublicKeyArray(Array::new(&*content, heap)?))
    }
}

impl ToAscObj<AscTransactionArray> for Vec<codec::NeoTransaction> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscTransactionArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x)).collect();
        let content = content?;
        Ok(AscTransactionArray(Array::new(&*content, heap)?))
    }
}


 

 



