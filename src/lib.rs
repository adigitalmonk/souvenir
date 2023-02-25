//! # Souvenir
//!
//! Memoization made easier.
//!
//! A collection of tools that handle the annoying parts of memoization.
//!
//! [Memory]: crate::memory::Memory
//! [Recall]: crate::recall::Recall

#![deny(clippy::pedantic)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![warn(clippy::style)]

mod memory;
mod recall;

pub use memory::Memory;
pub use recall::Recall;
