//! Standard Traits improve the interoperability between crates by defining a set of common functionality.
//!
//! For example, both `std::collections::HashMap` and `indexmap::IndexMap` have an `insert` method. However, we can't write a function that accepts both types, since there is no `Insert` trait. This crate provides a generic `Insert` trait & many others.
//!
//! This crate also provides implementations for types in `std` and other popular crates (for example: `indexmap`, `camino`).
//!
//! If you would like to implement the standard traits for your own types, please add `standard-traits` as a dependency and put the implementations in your crate (next to the types).
//!
//! ## Recommendations for trait definitions
//!
//! * Use a single verb
//!   * Good: `Add`
//!   * Bad: `Addition`
//! * Define a single method per trait
//! * Use the same name for the trait & for the method
//! * Use full names
//!   * Good: `Increment`
//!   * Bad: `Inc`
//! * Parametrize every type
//!   * Parametrize every input type via trait parameter
//!   * Parametrize the output type via associated type
//!     * Note that implementors can set `type Output = ()` for methods that shouldn't return anything (for example: mutators)
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
//! ## Recommendations for dependency definitions
//!
//! * Every dependency name must have a version suffix
//!   * If the dependency has a major version >= 1, then the dependency name must have a suffix that is equal to the major version
//!     * Good: `camino_1 = { package = "camino", version = "1.0.0", optional = true, default-features = false }`
//!     * Bad (suffix must not contain the minor and patch version): `camino_1_0_0 = { package = "camino", version = "1.0.0", optional = true, default-features = false }`
//!     * Bad (suffix must be present): `camino = { package = "camino", version = "1.0.0", optional = true, default-features = false }`
//!   * If the dependency has a major version == 0, then the dependency name must have a suffix that is equal to the major version and minor version
//!     * Good: `foo_0_1 = { package = "foo", version = "0.1.0", optional = true, default-features = false }`
//!     * Bad (suffix must contain the minor version): `foo_0 = { package = "foo", version = "0.1.0", optional = true, default-features = false }`
//!     * Bad (suffix must not contain the patch version): `foo_0_1_0 = { package = "foo", version = "0.1.0", optional = true, default-features = false }`
//!     * Bad (suffix must be present): `foo = { package = "foo", version = "0.1.0", optional = true, default-features = false }`
//! * Every dependency must have at least one entry in the `features` table
//!

// TODO: Ensure that every `mod` in a trait file has `pub` visibility (we need to publish the try_insert::impl_indexmap_2::OccupiedError)
// TODO: Looks like we have to abandon the global export system through `pub use ...::*`

#![deny(clippy::arithmetic_side_effects)]
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
mod push_ret_ref;
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
pub use push_ret_ref::*;
pub use trim::*;
pub use try_insert::*;

mod create_file_all;
mod join;
mod write_file_all;

pub use create_file_all::*;
pub use join::*;
pub use write_file_all::*;

mod map_self;

pub use map_self::*;

mod is_empty;

pub use is_empty::*;

mod append;
mod is_uppercase;

pub use is_uppercase::*;

pub use append::*;

mod opt_from;

pub use opt_from::*;
mod set;
