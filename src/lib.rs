#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod branch;
pub mod context;
pub mod reference;
pub mod repository;
pub mod tag;
