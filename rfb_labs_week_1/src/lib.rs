//! Rust for Bitcoin 2.0 — Week 1 practical lab skeleton.
//!
//! The RPC transport and shared data models are provided. Student work lives in
//! [`labs`], where every required function is intentionally unimplemented.

pub mod error;
pub mod labs;
pub mod model;
pub mod rpc;

pub use error::{LabError, LabResult};
