#![deny(clippy::all)]

pub mod commit;
mod error;
pub mod object;
pub mod reference;
pub mod remote;
pub mod repository;
pub mod revparse;
pub mod signature;
pub(crate) mod util;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
