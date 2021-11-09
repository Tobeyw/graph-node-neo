use graph::blockchain;
use graph::blockchain::Block;
use graph::blockchain::TriggerData;
use graph::cheap_clone::CheapClone;
use graph::prelude::hex;
use graph::prelude::web3::types::H256;
use graph::prelude::BlockNumber;
use graph::runtime::asc_new;
use graph::runtime::AscHeap;
use graph::runtime::AscPtr;
use graph::runtime::DeterministicHostError;
use std::{cmp::Ordering, sync::Arc};


use crate::codec::CryptoHash;
use crate::codec::{NeoBlock, RpcNotifyEventArgs as Event,RpcApplicationLog};

pub enum NeoTrigger {
    Block(Arc<NeoBlock>),
    Receipt(Arc<RpcApplicationLog>),
}

impl NeoTrigger {

    pub fn block_hash(&self) -> CryptoHash {
        match self {
            NeoTrigger::Block(block) => block.header.unwrap().hash.unwrap(),
            NeoTrigger::Receipt(receipt) => receipt.blockhash.unwrap()
        }
    }
}

impl TriggerData for NeoTrigger {
    fn error_context(&self) -> std::string::String {
        match self {
            NeoTrigger::Block(block) => {
                format!("Block ({:?})", block.header.unwrap().hash.unwrap() )
            }
            NeoTrigger::Receipt(receipt) => {
                format!(
                    "receipt id {:?}, block  ({:?})",
                    receipt.txid,
              
                    receipt.blockhash
                )
            }
        }
    }
}