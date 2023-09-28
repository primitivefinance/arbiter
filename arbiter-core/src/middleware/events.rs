use std::fmt::Debug;

/// ! Specific event types that can be emitted by the [`Environment`].
use ethers::types::Filter;

/// Packages together a [`crossbeam_channel::Receiver<Vec<Log>>`] along with a
/// [`Filter`] for events. Allows the client to have a stream of filtered
/// events.
#[derive(Debug)]
pub(crate) struct FilterReceiver {
    /// The filter definition used for this receiver.
    /// Comes from the `ethers-rs` crate.
    pub(crate) filter: Filter,

    /// The receiver for the channel that receives logs from the broadcaster.
    /// These are filtered upon reception.
    pub(crate) receiver: crossbeam_channel::Receiver<Vec<revm::primitives::Log>>,
}
