#![deny(clippy::all)]

mod error;
pub mod remote;
pub mod repository;
pub mod revwalk;
pub(crate) mod util;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
