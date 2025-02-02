#![deny(clippy::unwrap_used)]
// Requires nightly.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[cfg(feature = "component")]
pub mod component;
pub mod event;
pub mod state_key;

mod batch_swap_output_data;
mod circuit_breaker;
mod swap_execution;
mod trading_pair;

pub use batch_swap_output_data::BatchSwapOutputData;
pub(crate) use circuit_breaker::ExecutionCircuitBreaker;
pub use swap_execution::SwapExecution;
pub use trading_pair::{DirectedTradingPair, DirectedUnitPair, TradingPair, TradingPairVar};

pub mod lp;
pub mod swap;
pub mod swap_claim;

pub use lp::action::{PositionClose, PositionOpen, PositionWithdraw};
pub use swap::Swap;
pub use swap_claim::SwapClaim;
