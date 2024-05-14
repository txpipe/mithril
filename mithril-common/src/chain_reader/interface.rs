use std::path::Path;

use async_trait::async_trait;
use pallas_traverse::MultiEraBlock;
use thiserror::Error;

use crate::{
    chain_observer::PallasChainObserver, entities::ChainPoint, CardanoNetwork, StdError, StdResult,
};

/// The type of a block in the chain
#[derive(Debug)]
pub struct ChainSyncBlock {
    /// The block bytes
    pub bytes: Vec<u8>,
}
impl ChainSyncBlock {
    /// Create a new [ChainSyncBlock]
    pub fn decode(&self) -> std::result::Result<MultiEraBlock<'_>, pallas_traverse::Error> {
        pallas_traverse::MultiEraBlock::decode(&self.bytes)
    }
}

/// The error type for [ChainBlockReader]
#[derive(Debug, Error)]
pub enum ChainBlockReaderError {
    /// Generic [ChainBlockReader] error.
    #[error("general error")]
    General(#[source] StdError),
}

/// The action that indicates what to do next when scanning the chain
pub enum ChainBlockNextAction {
    /// RollForward event (we are still on the correct fork)
    RollForward {
        /// The next point in the chain to read
        next_point: ChainPoint,
        /// The parsed block
        block: ChainSyncBlock,
    },
    /// RollBackward event (we are on an incorrect fork, we need to get back a point to roll forward again)
    RollBackward {
        /// The rollback point in the chain to read (as a new valid point to read from on the main chain, which has already been seen)
        rollback_point: ChainPoint,
    },
    /// Do nothing event (we are on the correct fork, but we don't have any new blocks to read)
    DoNothing,
}

/// The trait that reads events to either:
/// read next block on the chain
/// rollback to another point in case of rollback
/// do nothing
#[async_trait]
pub trait ChainBlockReader: Send + Sync {
    /// Intersect the chain with the observer
    async fn intersect(&self, observer: PallasChainObserver) -> StdResult<()>;

    /// Get next chain block action
    async fn get_next_chain_block(
        &self,
        chain_point: pallas_network::miniprotocols::chainsync::NextResponse<
            pallas_network::miniprotocols::chainsync::BlockContent,
        >,
    ) -> StdResult<Option<ChainBlockNextAction>>;

    async fn sync(&self, socket_path: &Path, network: CardanoNetwork) -> StdResult<()> {
        // must be refactored
        // TBD: use the observer to get the current chain point plus intersect
        // or the client itself to get the next chain point
        let observer = PallasChainObserver::new(socket_path, network);
        // let chain_point = observer.get_current_chain_point().await.unwrap();

        let mut client = observer.get_client().await.unwrap();
        let chainsync = client.chainsync();

        self.intersect(observer).await?;

        match chainsync.has_agency() {
            true => {
                let next_point = chainsync
                    .request_next()
                    .await
                    .map_err(|e| ChainBlockReaderError::General(anyhow::Error::new(e)))?;
                self.get_next_chain_block(next_point).await?;
            }
            false => {
                let next_point = chainsync
                    .recv_while_must_reply()
                    .await
                    .map_err(|e| ChainBlockReaderError::General(anyhow::Error::new(e)))?;
                self.get_next_chain_block(next_point).await?;
            }
        }

        Ok(())
    }
}
