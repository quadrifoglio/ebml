//! ebml-rs
//!
//! ## Description
//!
//! This library aims to provide a simple and ergonomic way to read and write EBML documents.

#[macro_use]
extern crate error_chain;

#[macro_use]
pub mod element;

pub mod error;
pub mod header;
pub mod reader;

#[cfg(test)]
mod tests;
