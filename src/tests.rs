#![allow(clippy::wildcard_imports)]
use super::{SmallestUIntFor, SmallestUIntUpTo};
use core::any::TypeId;
use typenum::consts::*;
use typenum::Shleft;

fn id<T: 'static>() -> TypeId {
    TypeId::of::<T>()
}

type U2Pow64 = Shleft<U1, U64>;
#[cfg(feature = "u128")]
type U2Pow128 = Shleft<U1, U128>;

#[test]
fn test() {
    assert_eq!(id::<SmallestUIntUpTo<U0>>(), id::<u8>());
    assert_eq!(id::<SmallestUIntFor<U0>>(), id::<u8>());
    assert_eq!(id::<SmallestUIntUpTo<U1>>(), id::<u8>());
    assert_eq!(id::<SmallestUIntFor<U1>>(), id::<u8>());
    assert_eq!(id::<SmallestUIntUpTo<U255>>(), id::<u8>());
    assert_eq!(id::<SmallestUIntFor<U255>>(), id::<u8>());
    assert_eq!(id::<SmallestUIntUpTo<U256>>(), id::<u8>());
    assert_eq!(id::<SmallestUIntFor<U256>>(), id::<u16>());
    assert_eq!(id::<SmallestUIntUpTo<U257>>(), id::<u16>());
    assert_eq!(id::<SmallestUIntFor<U257>>(), id::<u16>());
    assert_eq!(id::<SmallestUIntUpTo<U65536>>(), id::<u16>());
    assert_eq!(id::<SmallestUIntFor<U65536>>(), id::<u32>());
    assert_eq!(id::<SmallestUIntUpTo<U4294967296>>(), id::<u32>());
    assert_eq!(id::<SmallestUIntFor<U4294967296>>(), id::<u64>());
    assert_eq!(id::<SmallestUIntUpTo<U2Pow64>>(), id::<u64>());
}

#[cfg(feature = "u128")]
#[test]
fn test_u128() {
    assert_eq!(id::<SmallestUIntFor<U2Pow64>>(), id::<u128>());
    assert_eq!(id::<SmallestUIntUpTo<U2Pow128>>(), id::<u128>());
}
