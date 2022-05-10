[![](https://img.shields.io/crates/v/unique-type)](https://crates.io/crates/unique-type)
[![](https://img.shields.io/docsrs/unique-type)](https://docs.rs/unique-type/latest/)
[![](https://img.shields.io/crates/l/unique-type)](https://github.com/Rimpampa/unique-type)

# Unique Type

This is (currently) a nightly only crate that provides some utilities
for generating and operating on unique and anonymous types.
In other words those are types that cannot be named and that are guardanteed
to always be different from every other type.

The main feature of this crate is the macro `new!` which can be used
to generate those kind of types.

The `Unique` trait can be used in trait bounds for requiring a type
to be generated from the `new!` macro.

Those types can then be used to "tag" other types to make them uniquely identifiable.

## Current implementation

The current implementation is based on the fact that
(from the [Rust Reference](https://doc.rust-lang.org/reference/types/closure.html))
a closure expression produces a closure value with a unique, anonymous type that cannot be written out

Basically the `new!` macro takes the `TypeId` of a closure and uses that as a const generic
to a "template" type that implements the `Unique` trait.

## Safety

The main problem of this approach is that the template type and all the other
types it depends on have to be publicly visible in order to be constructed inside the macro.

This means that one could declare such a type manually, possibly breaking the uniqueness guarantee.

This is solved by require `unsafe` code to initialize the template type where the Safety section
clearly states the requirements to match for it to be considered unique.

An additional protection is added by using the `#[doc(hidden)]` on those items that require
more attention (and that in theory shouldn't be visibile outside the crate).