
use crate::codec::{self, NeoBlockHeader, NeoTransaction};
use crate::trigger::ReceiptWithOutcome;
use base64;
use graph::anyhow::{anyhow, Context};
use graph::runtime::{asc_new, AscHeap, AscPtr, DeterministicHostError, ToAscObj,FromAscObj};
use graph_runtime_wasm::asc_abi::class::{Array, AscEnum, EnumPayload, Uint8Array};

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

 

impl ToAscObj<AscNeoBlockHeader> for NeoBlockHeader {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscNeoBlockHeader, DeterministicHostError> {
        Ok(AscNeoBlockHeader {
            hash:asc_new(heap,&self.hash)?,
            size: self.size,
            version: asc_new(heap,&self.version)?,
            previous_block_hash: asc_new(heap,&self.previous_block_hash)?,
            merkle_root: asc_new(heap,&self.merkrle_root)?,
            time:  asc_new(heap,&self.time)?,
            nonce: asc_new(heap,&self.nonce)?,
            primary:  asc_new(heap,&self.primary)?,
            witnesses:  asc_new(heap,&self.witnesses)?,
            next_consensus: asc_new(heap,&self.nextconsensus)?,
            next_block_hash:  asc_new(heap,&self.nextblockhash)?,
            confirmation: asc_new(heap,&self.confirmations)?,
        })
    }
}

 

impl ToAscObj<AscCryptoHash> for codec::CryptoHash {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
    ) -> Result<AscCryptoHash, DeterministicHostError> {
       
        self.hash.to_asc_obj(heap)
    }
}


