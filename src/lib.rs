//! # graph-algorithms
//!
//! Graph algorithm library — structures, algorithms, and spectral methods
//! for network analysis.

pub mod graph;
pub mod traversal;
pub mod shortest_path;
pub mod mst;
pub mod flow;
pub mod connectivity;
pub mod matching;
pub mod spectral;
pub mod network;

#[cfg(test)]
mod integration_tests;

pub use graph::{Graph, Edge};
