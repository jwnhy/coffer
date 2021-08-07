#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "num")]
extern crate num;

use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct LittleEndian<T>(T);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BigEndian<T>(T);

macro_rules! impl_endian {
    ($type_name:ident) => {
        impl BigEndian<$type_name> {
            pub const fn from_native(data: $type_name) -> Self {
                Self(data.to_be())
            }

            pub const fn new(data: $type_name) -> Self {
                Self(data)
            }

            pub fn to_native(&self) -> $type_name {
                match () {
                    #[cfg(target_endian = "big")]
                    () => self.0,
                    #[cfg(target_endian = "little")]
                    () => self.0.swap_bytes(),
                }
            }
        }

        impl LittleEndian<$type_name> {
            pub const fn from_native(data: $type_name) -> Self {
                Self(data.to_le())
            }
            pub const fn new(data: $type_name) -> Self {
                Self(data)
            }

            pub fn to_native(&self) -> $type_name {
                match () {
                    #[cfg(target_endian = "big")]
                    () => self.0.swap_bytes(),
                    #[cfg(target_endian = "little")]
                    () => self.0,
                }
            }
        }

        impl From<BigEndian<$type_name>> for $type_name {
            #[inline]
            fn from(data: BigEndian<$type_name>) -> Self {
                data.to_native()
            }
        }

        impl From<LittleEndian<$type_name>> for $type_name {
            #[inline]
            fn from(data: LittleEndian<$type_name>) -> Self {
                data.to_native()
            }
        }

        impl From<$type_name> for BigEndian<$type_name> {
            #[inline]
            fn from(data: $type_name) -> Self {
                Self(data.to_be())
            }
        }

        impl From<$type_name> for LittleEndian<$type_name> {
            #[inline]
            fn from(data: $type_name) -> Self {
                Self(data.to_le())
            }
        }

        impl From<LittleEndian<$type_name>> for BigEndian<$type_name> {
            #[inline]
            fn from(data: LittleEndian<$type_name>) -> Self {
                Self(data.0.swap_bytes())
            }
        }

        impl From<BigEndian<$type_name>> for LittleEndian<$type_name> {
            #[inline]
            fn from(data: BigEndian<$type_name>) -> Self {
                Self(data.0.swap_bytes())
            }
        }

        impl BitAnd<LittleEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            #[inline]
            fn bitand(self, rhs: LittleEndian<$type_name>) -> Self {
                BigEndian(self.0 & rhs.0.swap_bytes())
            }
        }

        impl BitAnd<BigEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            #[inline]
            fn bitand(self, rhs: BigEndian<$type_name>) -> Self {
                BigEndian(self.0 & rhs.0)
            }
        }

        impl BitOr<LittleEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            #[inline]
            fn bitor(self, rhs: LittleEndian<$type_name>) -> Self {
                BigEndian(self.0 | rhs.0.swap_bytes())
            }
        }

        impl BitOr<BigEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            #[inline]
            fn bitor(self, rhs: BigEndian<$type_name>) -> Self {
                BigEndian(self.0 | rhs.0)
            }
        }

        impl BitXor<LittleEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            #[inline]
            fn bitxor(self, rhs: LittleEndian<$type_name>) -> Self {
                BigEndian(self.0 ^ rhs.0.swap_bytes())
            }
        }

        impl BitXor<BigEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            #[inline]
            fn bitxor(self, rhs: BigEndian<$type_name>) -> Self {
                BigEndian(self.0 ^ rhs.0)
            }
        }

        impl PartialEq<$type_name> for BigEndian<$type_name> {
            #[inline]
            fn eq(&self, other: &$type_name) -> bool {
                self.0.eq(&other.to_be())
            }
        }

        impl PartialEq<BigEndian<$type_name>> for $type_name {
            #[inline]
            fn eq(&self, other: &BigEndian<$type_name>) -> bool {
                self.to_be().eq(&other.0)
            }
        }

        impl PartialOrd<$type_name> for BigEndian<$type_name> {
            #[inline]
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> {
                Some(self.0.cmp(&other.to_be()))
            }
        }

        impl PartialOrd<BigEndian<$type_name>> for $type_name {
            #[inline]
            fn partial_cmp(&self, other: &BigEndian<$type_name>) -> Option<Ordering> {
                Some(self.to_be().cmp(&other.0))
            }
        }

        impl PartialEq<$type_name> for LittleEndian<$type_name> {
            #[inline]
            fn eq(&self, other: &$type_name) -> bool {
                self.0.eq(&other.to_le())
            }
        }

        impl PartialEq<LittleEndian<$type_name>> for $type_name {
            #[inline]
            fn eq(&self, other: &LittleEndian<$type_name>) -> bool {
                self.to_le().eq(&other.0)
            }
        }

        impl PartialOrd<$type_name> for LittleEndian<$type_name> {
            #[inline]
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> {
                Some(self.0.cmp(&other.to_le()))
            }
        }

        impl PartialOrd<LittleEndian<$type_name>> for $type_name {
            #[inline]
            fn partial_cmp(&self, other: &LittleEndian<$type_name>) -> Option<Ordering> {
                Some(self.to_le().cmp(&other.0))
            }
        }

        impl Add<$type_name> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            fn add(self, other: $type_name) -> Self::Output {
                BigEndian(self.0 + other.to_be())
            }
        }

        impl Add<BigEndian<$type_name>> for $type_name {
            type Output = $type_name;
            fn add(self, other: BigEndian<$type_name>) -> Self::Output {
                self.to_be() + other.0
            }
        }

        impl Add<BigEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            fn add(self, other: BigEndian<$type_name>) -> Self::Output {
                BigEndian(self.0 + other.0)
            }
        }

        impl Add<$type_name> for LittleEndian<$type_name> {
            type Output = LittleEndian<$type_name>;
            fn add(self, other: $type_name) -> Self::Output {
                LittleEndian(self.0 + other.to_le())
            }
        }

        impl Add<LittleEndian<$type_name>> for $type_name {
            type Output = $type_name;
            fn add(self, other: LittleEndian<$type_name>) -> Self::Output {
                self.to_le() + other.0
            }
        }

        impl Add<LittleEndian<$type_name>> for LittleEndian<$type_name> {
            type Output = LittleEndian<$type_name>;
            fn add(self, other: LittleEndian<$type_name>) -> Self::Output {
                LittleEndian(self.0 + other.0)
            }
        }

        impl Sub<$type_name> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            fn sub(self, other: $type_name) -> Self::Output {
                BigEndian(self.0 - other.to_be())
            }
        }

        impl Sub<BigEndian<$type_name>> for $type_name {
            type Output = $type_name;
            fn sub(self, other: BigEndian<$type_name>) -> Self::Output {
                self.to_be() - other.0
            }
        }

        impl Sub<BigEndian<$type_name>> for BigEndian<$type_name> {
            type Output = BigEndian<$type_name>;
            fn sub(self, other: BigEndian<$type_name>) -> Self::Output {
                BigEndian(self.0 - other.0)
            }
        }

        impl Sub<$type_name> for LittleEndian<$type_name> {
            type Output = LittleEndian<$type_name>;
            fn sub(self, other: $type_name) -> Self::Output {
                LittleEndian(self.0 - other.to_le())
            }
        }

        impl Sub<LittleEndian<$type_name>> for $type_name {
            type Output = $type_name;
            fn sub(self, other: LittleEndian<$type_name>) -> Self::Output {
                self.to_le() - other.0
            }
        }

        impl Sub<LittleEndian<$type_name>> for LittleEndian<$type_name> {
            type Output = LittleEndian<$type_name>;
            fn sub(self, other: LittleEndian<$type_name>) -> Self::Output {
                LittleEndian(self.0 - other.0)
            }
        }
    };
}

impl_endian!(u8);
impl_endian!(u16);
impl_endian!(u32);
impl_endian!(u64);
impl_endian!(u128);
impl_endian!(usize);
impl_endian!(i8);
impl_endian!(i16);
impl_endian!(i32);
impl_endian!(i64);
impl_endian!(i128);
impl_endian!(isize);

pub mod types {
    pub type u8_le = super::LittleEndian<u8>;
    pub type u16_le = super::LittleEndian<u16>;
    pub type u32_le = super::LittleEndian<u32>;
    pub type u64_le = super::LittleEndian<u64>;
    pub type u128_le = super::LittleEndian<u128>;
    pub type usize_le = super::LittleEndian<usize>;
    pub type i8_le = super::LittleEndian<i8>;
    pub type i16_le = super::LittleEndian<i16>;
    pub type i32_le = super::LittleEndian<i32>;
    pub type i64_le = super::LittleEndian<i64>;
    pub type i128_le = super::LittleEndian<i128>;
    pub type isize_le = super::LittleEndian<isize>;
    pub type u8_be = super::BigEndian<u8>;
    pub type u16_be = super::BigEndian<u16>;
    pub type u32_be = super::BigEndian<u32>;
    pub type u64_be = super::BigEndian<u64>;
    pub type u128_be = super::BigEndian<u128>;
    pub type usize_be = super::BigEndian<usize>;
    pub type i8_be = super::BigEndian<i8>;
    pub type i16_be = super::BigEndian<i16>;
    pub type i32_be = super::BigEndian<i32>;
    pub type i64_be = super::BigEndian<i64>;
    pub type i128_be = super::BigEndian<i128>;
    pub type isize_be = super::BigEndian<isize>;
}
