//! # ethportal-api
//!
//! `ethportal_api` is a collection of Portal Network APIs and types.
#![warn(clippy::unwrap_used)]
#![warn(clippy::uninlined_format_args)]

extern crate lazy_static;

mod eth;
#[cfg(test)]
mod test_utils;
pub mod types;
pub mod utils;
pub mod version;
mod web3;

pub use eth::{EthApiClient, EthApiServer};
pub use types::{
    consensus,
    consensus::light_client,
        execution::{block_body::*, receipts::*},
    protocol_info::*,
    protocol_versions::*,
};
pub use web3::{Web3ApiClient, Web3ApiServer};
