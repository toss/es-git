#![deny(clippy::all)]

pub mod annotated_commit;
pub mod blame;
pub mod blob;
pub mod branch;
pub mod checkout;
pub mod commit;
pub mod config;
pub mod diff;
mod error;
pub mod ignore;
pub mod index;
pub mod mailmap;
pub mod merge;
pub mod object;
pub mod oid;
pub mod reference;
pub mod remote;
pub mod repository;
pub mod revparse;
pub mod revwalk;
pub mod signature;
pub mod status;
pub mod tag;
pub mod tree;
pub(crate) mod util;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
