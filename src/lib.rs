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

/// A set of values that can only be constructed through the
/// [`Set::unique`] function
#[doc(hidden)]
#[derive(PartialEq, Eq)]
pub struct Set(u32, u32, &'static str);

impl Set {
    /// Constructs a new set of values that are unique from any other
    /// generated with this function
    ///
    /// # Safety
    ///
    /// This function is safe only if:
    /// - `C` is [`std::column!`]
    /// - `L` is [`std::line!`]
    /// - `F` is [`std::file!`]
    pub const unsafe fn unique<const C: u32, const L: u32, const F: &'static str>() -> Self {
        Self(C, L, F)
    }
}

/// A template-like type for generating unique types
///
/// The uniqueness of this type is based on the [`Set`] type which
/// can only be safe constructed with unique values thus making every
/// "specialization" of this type generate a different type-id
#[doc(hidden)]
pub struct Template<const T: Set>(());

impl<const T: Set> pvt::Unique for Template<T> {}

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
        $crate::Template<{
            // SAFETY: the const generics values are the one stated in the docs for Set
            unsafe {
                $crate::Set::unique::<{ std::column!() }, { std::line!() }, { std::file!() }>()
            }
        }>
    };
}
