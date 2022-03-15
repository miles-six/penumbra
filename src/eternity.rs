use hash_hasher::HashedMap;
use thiserror::Error;

use crate::internal::{active::Forget as _, path::Witness as _};
use crate::*;

#[path = "epoch.rs"]
pub mod epoch;
use epoch::EpochMut;
pub use epoch::{block, Block, Epoch};

mod proof;
pub use proof::{Proof, VerifiedProof, VerifyError};

/// A sparse commitment tree to witness up to 65,536 [`Epoch`]s, each witnessing up to 65,536
/// [`Block`]s, each witnessing up to 65,536 [`Commitment`]s or their [`struct@Hash`]es.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Eternity {
    index: HashedMap<Commitment, index::within::Eternity>,
    inner: Tier<Tier<Tier<Item>>>,
}

/// The root hash of an [`Eternity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Root(Hash);

impl Height for Eternity {
    type Height = <Tier<Tier<Tier<Item>>> as Height>::Height;
}

/// An error occurred when trying to insert an item into an [`Eternity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum InsertError {
    /// The [`Eternity`] was full.
    #[error("eternity is full")]
    #[non_exhaustive]
    Full,
    /// The most recent [`Epoch`] of the [`Eternity`] was full.
    #[error("most recent epoch in eternity is full")]
    EpochFull,
    /// The most recent [`Epoch`] of the [`Eternity`] was forgotten.
    #[error("most recent epoch in eternity was forgotten")]
    EpochForgotten,
    /// The most recent [`Block`] of the most recent [`Epoch`] of the [`Eternity`] was full.
    #[error("most recent block in most recent epoch of eternity is full")]
    BlockFull,
    /// The most recent [`Block`] of the most recent [`Epoch`] of the [`Eternity`] was forgotten.
    #[error("most recent block in most recent epoch of eternity was forgotten")]
    BlockForgotten,
}

/// An error occurred when trying to insert a [`Block`] root into the [`Eternity`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InsertBlockError {
    /// The [`Eternity`] was full.
    #[error("eternity is full")]
    Full(Block),
    /// The most recent [`Epoch`] of the [`Eternity`] was full.
    #[error("most recent epoch is full")]
    EpochFull(Block),
    /// The most recent [`Epoch`] of the [`Eternity`] was forgotten.
    #[error("most recent epoch was forgotten")]
    EpochForgotten(Block),
}

/// An error occurred when trying to insert a [`Block`] root into the [`Eternity`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InsertBlockRootError {
    /// The [`Eternity`] was full.
    #[error("eternity is full")]
    Full,
    /// The most recent [`Epoch`] of the [`Eternity`] was full.
    #[error("most recent epoch is full")]
    EpochFull,
    /// The most recent [`Epoch`] of the [`Eternity`] was forgotten.
    #[error("most recent epoch was forgotten")]
    EpochForgotten,
}

/// The [`Eternity`] was full when trying to insert an [`Epoch`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("eternity is full")]
pub struct InsertEpochError(pub Epoch);

/// The [`Eternity`] was full when trying to insert an [`Epoch`] root.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("eternity is full")]
pub struct InsertEpochRootError;

impl Eternity {
    /// Create a new empty [`Eternity`] for storing all commitments to the end of time.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the root hash of this [`Eternity`].
    ///
    /// Internal hashing is performed lazily to prevent unnecessary intermediary hashes from being
    /// computed, so the first hash returned after a long sequence of insertions may take more time
    /// than subsequent calls.
    ///
    /// Computed hashes are cached so that subsequent calls without further modification are very
    /// fast.
    pub fn root(&self) -> Root {
        Root(self.inner.hash())
    }

    /// Add a new [`Commitment`] to the most recent [`Block`] of the most recent [`Epoch`] of this
    /// [`Eternity`].
    ///
    /// # Errors
    ///
    /// Returns `Err(commitment)` containing the inserted block without adding it to the [`Eternity`] if
    /// the [`Eternity`] is full, or the most recently inserted [`Epoch`] is full or was inserted by
    /// [`insert_epoch_root`](Eternity::insert_epoch_root), or the most recently inserted [`Block`]
    /// is full or was inserted by [`insert_block_root`](Eternity::insert_block_root).
    pub fn insert(&mut self, witness: Witness, commitment: Commitment) -> Result<(), Commitment> {
        self.insert_commitment_or_root(match witness {
            Keep => Insert::Keep(commitment),
            Forget => Insert::Hash(Hash::of(commitment)),
        })
        .map_err(|_| commitment)
    }

    /// Get a [`Proof`] of inclusion for the item at this index in the eternity.
    ///
    /// If the index is not witnessed in this eternity, return `None`.
    pub fn witness(&self, item: Commitment) -> Option<Proof> {
        let index = *self.index.get(&item)?;

        let (auth_path, leaf) = self.inner.witness(index)?;
        debug_assert_eq!(leaf, Hash::of(item));

        Some(Proof(crate::proof::Proof {
            index: index.into(),
            auth_path,
            leaf: item,
        }))
    }

    /// Forget about the witness for the given [`Commitment`].
    ///
    /// Returns `true` if the item was previously witnessed (and now is forgotten), and `false` if
    /// it was not witnessed.
    pub fn forget(&mut self, item: Commitment) -> bool {
        let mut forgotten = false;

        if let Some(&within_epoch) = self.index.get(&item) {
            // We forgot something
            forgotten = true;
            // Forget the index for this element in the tree
            let forgotten = self.inner.forget(within_epoch);
            debug_assert!(forgotten);
            // Remove this entry from the index
            self.index.remove(&item);
        }

        forgotten
    }

    /// Insert an item or its root (helper function for [`insert`].
    fn insert_commitment_or_root(&mut self, item: Insert<Commitment>) -> Result<(), InsertError> {
        // If the eternity is empty, we need to create a new epoch to insert the item into
        if self.inner.is_empty() && self.insert_epoch(Epoch::new()).is_err() {
            return Err(InsertError::Full);
        }

        match self.update(|epoch| {
            if let Some(epoch) = epoch {
                epoch.insert(item).map_err(|err| match err {
                    epoch::InsertError::Full => InsertError::EpochFull,
                    epoch::InsertError::BlockFull => InsertError::BlockFull,
                    epoch::InsertError::BlockForgotten => InsertError::BlockForgotten,
                })
            } else {
                Err(InsertError::EpochForgotten)
            }
        }) {
            Err(err) => Err(err),
            Ok(None) => Ok(()),
            Ok(Some(replaced)) => {
                // If inserting this item replaced some other item, forget the replaced index
                let forgotten = self.inner.forget(replaced);
                debug_assert!(forgotten);
                Ok(())
            }
        }
    }

    /// Add a new [`Block`] all at once to the most recently inserted [`Epoch`] of this
    /// [`Eternity`].
    ///
    /// # Errors
    ///
    /// Returns [`InsertBlockError`] containing the inserted block without adding it to the
    /// [`Eternity`] if the [`Eternity`] is full, or the most recently inserted [`Epoch`] is full or
    /// was inserted by [`Insert::Hash`].
    pub fn insert_block(&mut self, block: Block) -> Result<(), InsertBlockError> {
        // If the eternity is empty, we need to create a new epoch to insert the block into
        if self.inner.is_empty() && self.insert_epoch(Epoch::new()).is_err() {
            return Err(InsertBlockError::Full(block));
        }

        match self.update(|epoch| {
            if let Some(epoch) = epoch {
                epoch
                    .insert_block_or_root(Insert::Keep(block))
                    .map_err(|err| {
                        InsertBlockError::EpochFull(err.keep().expect(
                            "block must be returned in error case of `EpochMut::insert_block`",
                        ))
                    })
            } else {
                Err(InsertBlockError::EpochForgotten(block))
            }
        }) {
            Err(err) => Err(err),
            Ok(replaced) => {
                // When inserting the block, some indices in the block may overwrite existing
                // indices; we now can forget those indices because they're inaccessible
                for replaced in replaced {
                    let forgotten = self.inner.forget(replaced);
                    debug_assert!(forgotten);
                }
                Ok(())
            }
        }
    }

    /// Add the root hash of an [`Block`] to this [`Eternity`], without inserting any of the
    /// witnessed items in that [`Block`].
    ///
    /// # Errors
    ///
    /// Returns [`InsertBlockRootError`] if the [`Eternity`] is full, or the most recently inserted
    /// [`Epoch`] is full or was inserted by [`insert_epoch_root`](Eternity::insert_epoch_root).
    pub fn insert_block_root(
        &mut self,
        block_root: block::Root,
    ) -> Result<(), InsertBlockRootError> {
        // If the eternity is empty, we need to create a new epoch to insert the block into
        if self.inner.is_empty() && self.insert_epoch(Epoch::new()).is_err() {
            return Err(InsertBlockRootError::Full);
        }

        match self.update(|epoch| {
            if let Some(epoch) = epoch {
                epoch
                    .insert_block_or_root(Insert::Hash(block_root.0))
                    .map_err(|_| InsertBlockRootError::EpochFull)
            } else {
                Err(InsertBlockRootError::EpochForgotten)
            }
        }) {
            Err(err) => Err(err),
            Ok(replaced) => {
                // When inserting the block, some indices in the block may overwrite existing
                // indices; we now can forget those indices because they're inaccessible
                for replaced in replaced {
                    let forgotten = self.inner.forget(replaced);
                    debug_assert!(forgotten);
                }
                Ok(())
            }
        }
    }

    /// Add a new [`Epoch`] all at once to this [`Eternity`].
    ///
    /// # Errors
    ///
    /// Returns [`InsertEpochError`] containing the epoch without adding it to the [`Eternity`] if
    /// the [`Eternity`] is full.
    pub fn insert_epoch(&mut self, epoch: Epoch) -> Result<(), InsertEpochError> {
        self.insert_epoch_or_root(Insert::Keep(epoch))
            .map_err(|insert| {
                if let Insert::Keep(epoch) = insert {
                    InsertEpochError(epoch)
                } else {
                    unreachable!("failing to insert an epoch always returns the original epoch")
                }
            })
    }

    /// Add the root hash of an [`Epoch`] to this [`Eternity`], without inserting any of the
    /// witnessed items in that [`Epoch`].
    ///
    /// # Errors
    ///
    /// Returns [`InsertEpochRootError`] if the [`Eternity`] is full.
    pub fn insert_epoch_root(
        &mut self,
        epoch_root: epoch::Root,
    ) -> Result<(), InsertEpochRootError> {
        self.insert_epoch_or_root(Insert::Hash(epoch_root.0))
            .map_err(|insert| {
                if let Insert::Hash(_) = insert {
                    InsertEpochRootError
                } else {
                    unreachable!("failing to insert an epoch root always returns the original root")
                }
            })
    }

    /// Insert an epoch or its root (helper function for [`insert_epoch`] and [`insert_epoch_root`]).
    fn insert_epoch_or_root(&mut self, epoch: Insert<Epoch>) -> Result<(), Insert<Epoch>> {
        // If we successfully insert this epoch, here's what its index in the epoch will be:
        let this_epoch = self.inner.len().into();

        // Decompose the block into its components
        let (epoch, epoch_index) = match epoch {
            Insert::Hash(hash) => (Insert::Hash(hash), Default::default()),
            Insert::Keep(Epoch { index, inner }) => (Insert::Keep(inner), index),
        };

        // Try to insert the block into the tree, and if successful, track the item, block, and
        // epoch indices of each inserted item
        if let Err(epoch) = self.inner.insert(epoch) {
            Err(epoch.map(|inner| Epoch {
                index: epoch_index,
                inner,
            }))
        } else {
            for (
                item,
                index::within::Epoch {
                    block: this_block,
                    item: this_item,
                },
            ) in epoch_index.into_iter()
            {
                if let Some(replaced) = self.index.insert(
                    item,
                    index::within::Eternity {
                        epoch: this_epoch,
                        block: this_block,
                        item: this_item,
                    },
                ) {
                    // Forget the previous index of this inserted epoch, if there was one
                    self.inner.forget(replaced);
                }
            }

            Ok(())
        }
    }

    /// The total number of [`Commitment`]s or [`struct@Hash`]es represented in this [`Epoch`].
    ///
    /// This count includes those items which are elided due to a partially filled [`Block`] or
    /// [`Epoch`], or summary root [`struct@Hash`] of a block or epoch being inserted.
    ///
    /// The maximum capacity of an [`Eternity`] is 281,474,976,710,656 = 65,536 [`Epoch`]s of 65,536
    /// [`Block`]s of 65,536 [`Commitment`]s.
    pub fn len(&self) -> u64 {
        ((self.inner.len() as u64) << 32)
            + (match self.inner.focus() {
                None => 0,
                Some(Insert::Hash(_)) => u32::MAX,
                Some(Insert::Keep(epoch)) => {
                    (match epoch.focus() {
                        None => 0,
                        Some(Insert::Hash(_)) => u16::MAX,
                        Some(Insert::Keep(block)) => block.len(),
                    }) as u32
                }
            } << 16) as u64
    }

    /// Check whether this [`Eternity`] is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Update the most recently inserted [`Epoch`] via methods on [`EpochMut`], and return the
    /// result of the function.
    fn update<T>(&mut self, f: impl FnOnce(Option<&mut EpochMut<'_>>) -> T) -> T {
        let this_epoch = self.inner.len().saturating_sub(1).into();

        let index = epoch::IndexMut::Eternity {
            this_epoch,
            index: &mut self.index,
        };

        self.inner.update(|inner| {
            if let Some(inner) = inner {
                if let Insert::Keep(inner) = inner.as_mut() {
                    f(Some(&mut EpochMut { inner, index }))
                } else {
                    f(None)
                }
            } else {
                f(None)
            }
        })
    }
}
