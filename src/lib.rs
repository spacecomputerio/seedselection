//! # seedselection
//!
//! Deterministic, fair peer selection for distributed systems.
//!
//! This crate provides utilities for selecting a subset of peers from a set, in a way that is deterministic
//! (all honest nodes with the same input will select the same subset) and fair (selection is based on XOR distance
//! to a hash derived from a random seed, context name, and sequence number).
//!
//! ## Features
//! - Pluggable hash algorithm
//! - Selection based on XOR distance
//! - Efficient selection based on XOR distance, using a max-heap to choose the closest items
//!
//! ## Security
//! The selection process is designed to be secure against manipulation by malicious nodes, as it relies on a (pluggable) hash function and XOR distance which are difficult to predict or influence.
//! The selection depends on the entropy of the seed, so it is crucial to use a secure and unpredictable seed to ensure fairness and security.
//! We encourage users to use SpaceComputer's [orbitport](https://docs.spacecomputer.io/orbitport) as a source of randomness.
//!
//! ## Usage
//! use the `xor_distance_selection` function to select a subset of peers based on their IDs, a context name, a random seed, and a sequence number.
//! ```rust
//! use seedselection::xor_dist::xor_distance_selection;
//! use sha2::{Digest,Sha256};
//!
//! let name = "example_context";
//! let seed = b"random_seed";
//! let seq = 1;
//! let n = 5;
//! let peerset = vec![
//!     b"peer1",
//!     b"peer2",
//!     b"peer3",
//!     b"peer4",
//!     b"peer5",
//! ];
//! let selected_peers = xor_distance_selection(name, seed, seq, n, &peerset, Sha256::new, None).unwrap();
//! assert_eq!(selected_peers.len(), n);
//! ```

pub mod hash;
pub mod xor_dist;
