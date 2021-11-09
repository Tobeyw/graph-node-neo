use graph::blockchain::{Block, TriggerWithHandler};
use graph::components::store::StoredDynamicDataSource;
use graph::data::subgraph::DataSourceContext;
use graph::prelude::SubgraphManifestValidationError;
use graph::{
    anyhow::{anyhow, Error},
    blockchain::{self, Blockchain},
    prelude::{
        async_trait, info, BlockNumber, CheapClone, DataSourceTemplateInfo, Deserialize, Link,
        LinkResolver, Logger,
    },
    semver,
};
use std::collections::BTreeMap;
use std::{convert::TryFrom, sync::Arc};

//use crate::chain::Chain;
use crate::trigger::NeoTrigger;

/// Runtime representation of a data source.
#[derive(Clone, Debug)]
pub struct DataSource {
    pub kind: String,
    pub network: Option<String>,
    pub name: String,
    pub(crate) source: Source,
    pub mapping: Mapping,
    pub context: Arc<Option<DataSourceContext>>,
    pub creation_block: Option<BlockNumber>,
}





impl DataSource {
    fn from_manifest(
        kind: String,
        network: Option<String>,
        name: String,
        source: Source,
        mapping: Mapping,
        context: Option<DataSourceContext>,
    ) -> Result<Self, Error> {
        // Data sources in the manifest are created "before genesis" so they have no creation block.
        let creation_block = None;

        Ok(DataSource {
            kind,
            network,
            name,
            source,
            mapping,
            context: Arc::new(context),
            creation_block,
        })
    }

    fn handler_for_block(&self) -> Option<&MappingBlockHandler> {
        self.mapping.block_handlers.first()
    }

    fn handler_for_receipt(&self) -> Option<&ReceiptHandler> {
        self.mapping.receipt_handlers.first()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct UnresolvedDataSource {
    pub kind: String,
    pub network: Option<String>,
    pub name: String,
    pub(crate) source: Source,
    pub mapping: UnresolvedMapping,
    pub context: Option<DataSourceContext>,
}

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolvedMapping {
    pub api_version: String,
    pub language: String,
    pub entities: Vec<String>,
    #[serde(default)]
    pub block_handlers: Vec<MappingBlockHandler>,
    #[serde(default)]
    pub receipt_handlers: Vec<ReceiptHandler>,
    pub file: Link,
}


#[derive(Clone, Debug)]
pub struct Mapping {
    pub api_version: semver::Version,
    pub language: String,
    pub entities: Vec<String>,
    pub block_handlers: Vec<MappingBlockHandler>,
    pub receipt_handlers: Vec<ReceiptHandler>,
    pub runtime: Arc<Vec<u8>>,
    pub link: Link,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct MappingBlockHandler {
    pub handler: String,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct ReceiptHandler {
    handler: String,
}
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub(crate) struct Source {
    // A data source that does not have an account can only have block handlers.
    pub(crate) account: Option<String>,
    #[serde(rename = "startBlock", default)]
    pub(crate) start_block: BlockNumber,
}
