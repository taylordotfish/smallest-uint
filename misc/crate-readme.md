smallest-uint
=============

Given an unsigned [type-level integer][typenum], provides access to
the smallest primitive unsigned integer type ([`u8`], [`u16`], etc.) that
can represent the integer’s value (or all values less than that value).

This is mainly useful for minimizing memory usage when building data
structures.

This crate depends only on [`core`], so it can be used in `no_std`
environments.

[`core`]: https://doc.rust-lang.org/core/

Example
-------

```rust
use smallest_uint::{SmallestUIntFor, SmallestUIntUpTo};
use std::any::TypeId;
use typenum::U65536;

assert_eq!(TypeId::of::<SmallestUIntUpTo<U65536>>(), TypeId::of::<u16>());
assert_eq!(TypeId::of::<SmallestUIntFor<U65536>>(), TypeId::of::<u32>());
```

Crate features
--------------

If the feature `u128` is enabled, this crate will use the [`u128`] type.
This feature is enabled by default.

[typenum]: https://docs.rs/typenum
[`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
[`u16`]: https://doc.rust-lang.org/std/primitive.u16.html
[`u128`]: https://doc.rust-lang.org/std/primitive.u128.html
