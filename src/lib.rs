#![deny(clippy::all)]

pub mod blob;
pub mod commit;
pub mod config;
pub mod diff;
mod error;
pub mod index;
pub mod object;
pub mod oid;
pub mod reference;
pub mod remote;
pub mod repository;
pub mod revparse;
pub mod revwalk;
pub mod signature;
pub mod tag;
pub mod tree;
pub(crate) mod util;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
