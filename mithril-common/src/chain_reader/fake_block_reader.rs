use crate::entities::ChainPoint;

/// FakeBlockReader is a fake implementation of the BlockReader trait.
pub struct FakeBlockReader {
    /// The next point in the chain to read
    pub next_point: ChainPoint,

    /// The block to return
    pub block: Vec<u8>,
}

impl FakeBlockReader {
    /// Create a new FakeBlockReader
    pub fn new(next_point: ChainPoint, block: Vec<u8>) -> Self {
        Self { next_point, block }
    }
}
