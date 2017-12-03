//! # ebml-rs
//!
//! Simple Rust library to read & write EBML documents.
//!

#[macro_use]
extern crate error_chain;

pub mod error;
pub mod common;
pub mod reader;

#[cfg(test)]
mod tests;
