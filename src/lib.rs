//! Anvil world format — RegionFile reader/writer, chunk serialization, level.dat.
//!
//! This crate provides two main modules:
//! - [`anvil`]: Region file I/O, chunk loader/serializer, compression
//! - [`storage`]: World folder layout, level.dat, dirty tracking, dimension types

#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod anvil;
pub mod storage;
