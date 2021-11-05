use crate::trigger::ReceiptWithOutcome;
use base64;
use crate::codec;
use diesel::migration::Migration;
use graph::anyhow::{anyhow, Context};
use graph::runtime::{asc_new, AscHeap, AscPtr, DeterministicHostError, ToAscObj,FromAscObj};
use graph_runtime_wasm::asc_abi::class::{Array, AscAddress, AscEnum, EnumPayload, Uint8Array};

pub(crate) use super::generated::*;



impl ToAscObj<AscNeoBlock> for codec::NeoBlock{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscNeoBlock, DeterministicHostError> {  
         Ok(AscNeoBlock {
            header: asc_new(heap, &self.header)?,
            tx: asc_new(heap, &self.tx)?,           
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

//自定义基本类型
impl ToAscObj<AscCryptoHash> for codec::CryptoHash{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscCryptoHash, DeterministicHostError> {
       
        self.hash.to_asc_obj(heap)
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
 
impl ToAscObj<AscAddress> for codec::Address{
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscAddress, DeterministicHostError> {
       
        self.hash.to_asc_obj(heap)
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

 



