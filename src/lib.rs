//! # algorand-rs
//!
//! This crate is a WORK IN PROGRESS!
//!
//! **algorand-rs** aims at becoming a rusty algorand sdk.
//!
//! ```rust
//! use algorand_rs::algod::Algod;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let algod = Algod::new()
//!         .bind("http://localhost:4001")
//!         .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
//!         .client_v1()?;
//!
//!     println!("Algod versions: {:?}", algod.versions()?.versions);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Objectives
//!
//! - [ ] Example-driven API development
//! - [ ] Clear error messages
//! - [ ] Async requests
//! - [ ] Builder pattern and sensible defaults
//! - [ ] Thorough test suite
//! - [ ] Comprehensive documentation

// TODO #![deny(missing_docs)]

pub mod account;
/// Algorand protocol daemon
pub mod algod;
pub mod auction;
pub mod core;
pub mod crypto;
pub mod error;
/// Algorand's indexer
pub mod indexer;
/// Key management daemon
pub mod kmd;
/// Support for turning 32 byte keys into human-readable mnemonics and back
pub mod mnemonic;
pub(crate) mod serialization;
/// Api token management utils
pub(crate) mod token;
pub mod transaction;

// Re-exports
pub use crate::core::{MicroAlgos, Round};
pub use algod::Algod;
pub use crypto::Address;
pub use crypto::{HashDigest, MasterDerivationKey};
pub use indexer::Indexer;
pub use kmd::Kmd;
