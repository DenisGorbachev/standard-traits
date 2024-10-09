//! Standard Traits improve the interoperability between crates by defining a set of common functionality.
//!
//! For example, both [`std::collections::HashMap`] and [`indexmap_2::IndexMap`] have an `insert` method. However, we can't write a function that accepts both types, since there is no `Insert` trait. This crate provides
//!
//! This crate provides implementations for types in `std`. In addition, it provides implementations for existing popular crates (for example: `indexmap`, `camino`).
//!
//! If you would like to implement the standard traits for your own types, please add `standard-traits` as a dependency and put the implementations in your crate (next to the types).
//!
//! ## Recommendations for trait definitions
//!
//! * Use a single verb
//!   * Good: `Add`
//!   * Bad: `Addition`
//! * Define a single method per trait
//! * Use the same name for the method as for the trait
//! * Use full names
//!   * Good: `Increment`
//!   * Bad: `Inc`
//! * Parametrize every type
//!   * Parametrize input type via trait parameter
//!   * Parametrize output type via associated type
//! * Provide the Self type as a default value for the every trait parameter
//!
//! Good example:
//!
//! ```
//! pub trait Join<Rhs = Self> {
//!     type Output;
//!
//!     fn join(self, rhs: Rhs) -> Self::Output;
//! }
//! ```
//!
//! Bad example 1:
//!
//! ```
//! pub trait Join {
//!     type Output;
//!
//!     fn join(self, str: &str) -> Self::Output;
//! }
//! ```
//!
//! The trait definition is too narrow: the `join` function only accepts an `&str` as a second argument.
//!
//! Bad example 2:
//!
//! ```
//! pub trait Join<Rhs = Self> {
//!     fn join(self, rhs: Rhs) -> Self;
//! }
//! ```
//!
//! Suppose there is a type that can't implement `join` for any `rhs`, but it can implement for some `rhs`. In other words, `join` must return a `Result`. But this trait definition makes it impossible.
//!

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod contains;

#[cfg(feature = "unstable_get")]
mod get;
mod insert;
mod integration_tests;
mod len;
#[cfg(feature = "unstable_of")]
mod of;
mod provide;
mod push;
mod push_get_ref;
mod trim;
mod try_insert;

pub use contains::*;
#[cfg(feature = "unstable_get")]
pub use get::*;
pub use insert::*;
pub use len::*;
#[cfg(feature = "unstable_of")]
pub use of::*;
pub use provide::*;
pub use push::*;
pub use push_get_ref::*;
pub use trim::*;
pub use try_insert::*;

mod decrement;

pub use decrement::*;
