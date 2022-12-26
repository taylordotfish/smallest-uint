/*
 * Copyright 2021-2022 taylor.fish <contact@taylor.fish>
 *
 * This file is part of smallest-uint.
 *
 * smallest-uint is licensed under the Apache License, Version 2.0
 * (the "License"); you may not use smallest-uint except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![no_std]
#![cfg_attr(test, recursion_limit = "256")]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

//! Given an unsigned [type-level integer][typenum], provides access to
//! the smallest primitive unsigned integer type ([`u8`], [`u16`], etc.) that
//! can represent the integer’s value (or all values less than that value).
//!
//! This is mainly useful for minimizing memory usage when building data
//! structures.
//!
//! This crate depends only on [`core`], so it can be used in `no_std`
//! environments.
//!
//! [`core`]: https://doc.rust-lang.org/core/
//!
//! Example
//! -------
//!
//! ```rust
//! use smallest_uint::{SmallestUIntFor, SmallestUIntUpTo};
//! use std::any::TypeId;
//! use typenum::U65536;
//!
//! assert_eq!(TypeId::of::<SmallestUIntUpTo<U65536>>(), TypeId::of::<u16>());
//! assert_eq!(TypeId::of::<SmallestUIntFor<U65536>>(), TypeId::of::<u32>());
//! ```
//!
//! Crate features
//! --------------
//!
//! If the feature `u128` is enabled, this crate will use the [`u128`] type.
//! This feature is enabled by default.
//!
//! [typenum]: typenum

use num_traits::{NumAssign, PrimInt};
use typenum::Unsigned;

#[cfg(test)]
mod tests;

mod detail {
    use super::SmallestUIntFor;
    use core::ops::Sub;
    use num_traits::{NumAssign, PrimInt};
    use typenum::{Len, Max, Unsigned};
    use typenum::{Length, Maximum, Sub1};
    use typenum::{B1, U1};

    type Max1<N> = Maximum<N, U1>;

    pub trait GetSmallestUIntFor: Unsigned {
        type Type: Default + PrimInt + NumAssign;
    }

    impl<N> GetSmallestUIntFor for N
    where
        N: Unsigned + Max<U1>,
        Max1<N>: Len,
        Length<Max1<N>>: Sub<B1>,
        Sub1<Length<Max1<N>>>: Len,
        Length<Sub1<Length<Max1<N>>>>: Get,
    {
        type Type = <Length<Sub1<Length<Max1<N>>>> as Get>::Type;
    }

    pub trait GetSmallestUIntUpTo: Unsigned {
        type Type: Default + PrimInt + NumAssign;
    }

    impl<N> GetSmallestUIntUpTo for N
    where
        N: Unsigned + Max<U1>,
        Max1<N>: Sub<B1>,
        Sub1<Max1<N>>: super::GetSmallestUIntFor,
    {
        type Type = SmallestUIntFor<Sub1<Max1<N>>>;
    }

    /// Gets the smallest unsigned integer type with at least 2**`Self` bits.
    pub trait Get {
        type Type: Default + PrimInt + NumAssign;
    }

    impl Get for typenum::U0 {
        type Type = u8;
    }

    impl Get for typenum::U1 {
        type Type = u8;
    }

    impl Get for typenum::U2 {
        type Type = u8;
    }

    impl Get for typenum::U3 {
        type Type = u8;
    }

    impl Get for typenum::U4 {
        type Type = u16;
    }

    impl Get for typenum::U5 {
        type Type = u32;
    }

    impl Get for typenum::U6 {
        type Type = u64;
    }

    #[cfg(feature = "u128")]
    impl Get for typenum::U7 {
        type Type = u128;
    }
}

/// Gets the smallest unsigned integer type capable of representing the value
/// of `Self`.
///
/// `Self` is an unsigned [type-level integer](typenum).
///
/// The type is provided as the associated type [`Self::Type`]. If you just
/// want the type and aren’t specifying trait bounds, use the type alias
/// [`SmallestUIntFor`].
pub trait GetSmallestUIntFor: Unsigned + detail::GetSmallestUIntFor {
    /// The unsigned integer type.
    type Type: Default + PrimInt + NumAssign;
}

/// Alias of [`GetSmallestUIntFor::Type`].
pub type SmallestUIntFor<N> = <N as GetSmallestUIntFor>::Type;

impl<N> GetSmallestUIntFor for N
where
    N: detail::GetSmallestUIntFor,
{
    type Type = <N as detail::GetSmallestUIntFor>::Type;
}

/// Gets the smallest unsigned integer type capable of representing all values
/// up to, but not including, the value of `Self`.
///
/// `Self` is an unsigned [type-level integer](typenum).
///
/// The type is provided as the associated type [`Self::Type`]. If you just
/// want the type and aren’t specifying trait bounds, use the type alias
/// [`SmallestUIntUpTo`].
pub trait GetSmallestUIntUpTo: Unsigned + detail::GetSmallestUIntUpTo {
    /// The unsigned integer type.
    type Type: Default + PrimInt + NumAssign;
}

/// Alias of [`GetSmallestUIntUpTo::Type`].
pub type SmallestUIntUpTo<N> = <N as GetSmallestUIntUpTo>::Type;

impl<N> GetSmallestUIntUpTo for N
where
    N: detail::GetSmallestUIntUpTo,
{
    type Type = <N as detail::GetSmallestUIntUpTo>::Type;
}
