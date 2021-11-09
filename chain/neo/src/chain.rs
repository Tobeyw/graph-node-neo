use graph::blockchain::BlockchainKind;
use graph::cheap_clone::CheapClone;
use graph::data::subgraph::UnifiedMappingApiVersion;
use graph::firehose::endpoints::FirehoseNetworkEndpoints;
use graph::prelude::StopwatchMetrics;
use graph::{
    anyhow,
    blockchain::{
        block_stream::{
            BlockStreamEvent, BlockStreamMetrics, BlockWithTriggers, FirehoseError,
            FirehoseMapper as FirehoseMapperTrait, TriggersAdapter as TriggersAdapterTrait,
        },
        firehose_block_stream::FirehoseBlockStream,
        BlockHash, BlockPtr, Blockchain, IngestorAdapter as IngestorAdapterTrait, IngestorError,
    },
    components::store::DeploymentLocator,
    firehose::bstream,
    log::factory::{ComponentLoggerConfig, ElasticComponentLoggerConfig},
    prelude::{
        async_trait, o, BlockNumber, ChainStore, Error, Logger, LoggerFactory, SubgraphStore,
    },
};
use prost::Message;
use std::sync::Arc;

//use crate::adapter::TriggerFilter;
use crate::capabilities::NodeCapabilities;
//use crate::data_source::{DataSourceTemplate, UnresolvedDataSourceTemplate};
use crate::runtime::RuntimeAdapter;
use crate::trigger::{self, NeoTrigger};
use crate::{
    codec,
    data_source::{DataSource, UnresolvedDataSource},
};
use graph::blockchain::block_stream::BlockStream;

pub struct Chain {
    logger_factory: LoggerFactory,
    name: String,
    firehose_endpoints: Arc<FirehoseNetworkEndpoints>,
    chain_store: Arc<dyn ChainStore>,
    subgraph_store: Arc<dyn SubgraphStore>,
}

impl std::fmt::Debug for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chain: neo")
    }
}

impl Chain {
    pub fn new(
        logger_factory: LoggerFactory,
        name: String,
        chain_store: Arc<dyn ChainStore>,
        subgraph_store: Arc<dyn SubgraphStore>,
        firehose_endpoints: FirehoseNetworkEndpoints,
    ) -> Self {
        Chain {
            logger_factory,
            name,
            firehose_endpoints: Arc::new(firehose_endpoints),
            chain_store,
            subgraph_store,
        }
    }
}