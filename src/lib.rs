//! A set of traits that can be used to describe a generic blockchain.

// TODO: fallible decoding
trait ByteEncodable: Into<Vec<u8>> + for<'a> From<&'a [u8]> {}

/// A generic block. These contain transactions, a number, and a unique identifier.
trait Block: Sized + ByteEncodable {
    /// The type of transaction this kind of block stores.
    type Transaction: Clone;

    /// The type of unique identifier for this block, usually a hash.
    type Id: Eq;

    /// This block's parent, referred to by Id.
    fn parent(&self) -> Self::Id;

    /// This block's number. Assumed to start at 0, a genesis, and proceed incrementally from there.
    fn number(&self) -> u64;

    /// Get the identifier for this block.
    fn id(&self) -> Self::Id;

    /// The transactions contained in this block.
    fn transactions(&self) -> &[Self::Transaction];
}

/// A block which has uncles.
trait HasUncles: Block {
    /// The type of uncle this has.
    type Uncle;

    /// Get a list of uncle IDs.
    fn uncles(&self) -> Vec<Self::Uncle>;
}

/// A provider for block data.
trait BlockProvider {
    /// The block this kind of provider stores.
    type Block: Block;

    /// Whether a block is known, but not necessarily a

    /// Try to fetch raw block data by id.
    /// Returns `None` if it doesn't exist.
    fn block(&self, id: &<Self::Block as Block>::Id) -> Option<Self::Block>;

    /// Get the id for a given block number.
    /// Return `None` if it doesn't exist.
    fn block_id(&self, num: u64) -> Option<<Self::Block as Block>::Id>;

    /// Get the uncles for a given block.
    fn uncles(&self, id: &<Self::Block as Block>::Id) -> Option<Vec<<Self::Block as HasUncles>::Uncle>> where Self::Block: HasUncles {
        self.block(id).map(|b| b.uncles())
    }

    /// Get the transactions for a given block.
    fn transactions(&self, id: &<Self::Block as Block>::Id) -> Option<Vec<<Self::Block as Block>::Transaction>> {
        self.block(id).map(|b| b.transactions().to_vec())
    }
}

/// Verifier for a given type of block.
trait Verifier<B: Block> {
    /// The kind of errors which can occur during verification.
    type Error;

    /// Phase 1 verification: cheap checks on the block itself. Usually just to verify block integrity.
    fn verify_basic(&self, block: &B) -> Result<(), Self::Error>;

    /// Phase 2 verification: more expensive checks based on the block itself.
    /// This includes things like checking transaction signatures.
    fn verify_unordered(&self, block: &B) -> Result<(), Self::Error>;

    /// Phase 3 verification: perform checks based on this block as well as its "family".
    /// Different chains have different notions of block family, so this may include uncles,
    /// the parent block, or other ancestors.
    fn verify_family(&self, block: &B, provider: &BlockProvider<Block=B>) -> Result<(), Self::Error>;
}

/// The global state manipulated by blocks.
trait State {
    type Block: Block;
    type Error;

    /// enact a pre-verified block. In case of failure, changes must not be applied.
    fn enact(&mut self, block: Self::Block) -> Result<(), Self::Error>;
}

trait Chain {
    type Block: Block;
    type Verifier: Verifier<Self::Block>;
}