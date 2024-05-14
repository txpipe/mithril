use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};

use async_trait::async_trait;
use pallas_network::miniprotocols::{
    chainsync::{BlockContent, NextResponse},
    Point,
};
use pallas_traverse::MultiEraBlock;

use crate::{
    chain_observer::{ChainObserver, PallasChainObserver},
    entities::ChainPoint,
    CardanoNetwork, StdResult,
};

use super::interface::{
    ChainBlockNextAction, ChainBlockReader, ChainBlockReaderError, ChainSyncBlock,
};

/// Pallas chain reader
#[derive(Debug)]
pub struct PallasBlockReader {
    /// The block to return
    socket: PathBuf,

    /// The network
    network: CardanoNetwork,
}

impl From<anyhow::Error> for ChainBlockReaderError {
    fn from(err: anyhow::Error) -> Self {
        ChainBlockReaderError::General(err)
    }
}

impl PallasBlockReader {
    /// Create a new PallasBlockReader
    pub fn new(socket: &Path, network: CardanoNetwork) -> Self {
        Self {
            socket: socket.to_owned(),
            network,
        }
    }
}

#[async_trait]
impl ChainBlockReader for PallasBlockReader {
    /// Intersect the chain with the observer
    async fn intersect(&self, observer: PallasChainObserver) -> StdResult<()> {
        let mut client = observer.get_client().await?;
        let chainsync = client.chainsync();

        let chain_point = observer.get_current_chain_point().await.unwrap().unwrap();

        let known_points = vec![Point::Specific(
            chain_point.slot_number,
            hex::decode(&chain_point.block_hash).unwrap(),
        )];

        let (_point, _tip) = chainsync.find_intersect(known_points).await?;

        Ok(())
    }

    async fn get_next_chain_block(
        &self,
        chain_point: NextResponse<BlockContent>,
    ) -> StdResult<Option<ChainBlockNextAction>> {
        match chain_point {
            NextResponse::RollForward(cbor, tip) => {
                let block = MultiEraBlock::decode(&cbor)
                    .map_err(|err| anyhow!(err))
                    .with_context(|| "PallasChainObserver failed to get current era")?;

                let slot = block.slot();
                let hash = block.hash();

                let _block_point: Point = Point::Specific(slot, hash.to_vec());

                let header_hash = match &tip.0 {
                    Point::Origin => None,
                    Point::Specific(_at_slot, hash) => Some(hex::encode(hash)),
                };

                Ok(Some(ChainBlockNextAction::RollForward {
                    next_point: ChainPoint {
                        slot_number: tip.0.slot_or_default(),
                        block_hash: header_hash.unwrap_or_default(),
                        block_number: block.number(),
                    },
                    block: ChainSyncBlock {
                        bytes: cbor.to_vec(),
                    },
                }))
            }
            NextResponse::RollBackward(point, tip) => {
                let slot_number = point.slot_or_default();

                match &point {
                    Point::Origin => {
                        // TODO: rollback the state, implement it
                        // TODO: rollback anything else?
                        Ok(Some(ChainBlockNextAction::RollBackward {
                            rollback_point: ChainPoint {
                                slot_number,
                                block_hash: "".to_string(),
                                block_number: 0,
                            },
                        }))
                    }
                    Point::Specific(_at_slot, hash) => {
                        // TODO: rollback the state, implement it
                        // TODO: rollback anything else for the specific point?

                        Ok(Some(ChainBlockNextAction::RollBackward {
                            rollback_point: ChainPoint {
                                slot_number,
                                block_hash: hex::encode(hash),
                                block_number: tip.1,
                            },
                        }))
                    }
                }
            }
            pallas_network::miniprotocols::chainsync::NextResponse::Await => {
                Ok(Some(ChainBlockNextAction::DoNothing))
            }
        }
    }
}
