#![deny(clippy::all)]
#![allow(clippy::len_without_is_empty)]
pub mod annotated_commit;
pub mod apply;
pub mod blame;
pub mod blob;
pub mod branch;
pub mod checkout;
pub mod commit;
pub mod config;
pub mod describe;
pub mod diff;
mod error;
pub mod ignore;
pub mod index;
pub mod mailmap;
pub mod merge;
pub mod note;
pub mod object;
pub mod oid;
pub mod rebase;
pub mod reference;
pub mod remote;
pub mod repository;
pub mod revert;
pub mod revparse;
pub mod revwalk;
pub mod signature;
pub mod stash;
pub mod status;
pub mod tag;
pub mod tree;
pub(crate) mod util;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
