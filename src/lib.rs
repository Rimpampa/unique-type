//! # Unique Type
//!
//! The main feature of this crate is the macro [`new!`] which can be used
//! to generate special **unique** opaque types.
//! In other words those are types that cannot be named and that are guardanteed
//! to always be different from every other type.
//!
//! The [`Unique`] trait can be used in trait bounds for requiring a type
//! to be generated from that macro.

// Required for having &str as a const generic
#![feature(adt_const_params)]

mod pvt {
    /// Private version of [`Unique`](super::Unique)
    ///
    /// This seals the trait so that it cannot be implemented outside of the crate
    pub trait Unique {}
}

/// An interface for reqiring unique types
///
/// The only types implementing this trait are the ones generated from [`new!`].
pub trait Unique: pvt::Unique {}

impl<T: pvt::Unique> Unique for T {}

/// A template-like type for generating unique types
///
/// This type is guaranteed to be unique as long as the following restrictions apply:
/// - `C` has to be initialized with [`std::column!`]
/// - `L` has to be initialized with [`std::line!`]
/// - `F` has to be initialized with [`std::file!`]
#[doc(hidden)]
pub struct Template<const C: u32, const L: u32, const F: &'static str>(());

impl<const C: u32, const L: u32, const F: &'static str> pvt::Unique for Template<C, L, F> {}

/// Generates a unique type that implements the [`Unique`] trait
///
/// # Example
///
/// Calling this macro twice will always generate two different types,
/// thus this will panic:
/// ```should_panic
/// # fn main() {
/// # use std::any::TypeId;
/// assert_eq!(
///     TypeId::of::<unique_type::new!()>(),
///     TypeId::of::<unique_type::new!()>()
/// );
/// # }
/// ```
///
/// And this won't even compile:
/// ```compile_fail E0308
/// # fn main() {
/// let a: unique_type::new!() = todo!();
/// let b: unique_type::new!() = a;
/// # }
/// ```
#[macro_export]
macro_rules! new {
    () => {
        $crate::Template<{ std::column!() }, { std::line!() }, { std::file!() }>
    };
}
