#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod branch;
pub mod context;
mod error;
pub mod reference;
pub mod remote;
pub mod repository;
pub mod tag;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
