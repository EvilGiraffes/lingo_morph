pub use crate::monads::Either;
pub mod processors;
mod monads;
// This mimics the log crate to avoid checking for the feature available
#[macro_use]
mod log;

