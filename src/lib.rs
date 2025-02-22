#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(deprecated)]
#![allow(unused_must_use)]
#![allow(non_local_definitions)]
#[cfg(not(feature = "cargo-clippy"))]
// See https://github.com/SSheldon/rust-objc/pull/75 for updates on issues to do with compiler
// warnings caused by `ATOMIC_USIZE_INIT` being deprecated
mod error;
pub mod gatt;
mod peripheral;
mod uuid;

pub use self::{error::*, peripheral::Peripheral, uuid::*};
