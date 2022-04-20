//! TODO

extern crate doc_comment;
extern crate safe_transmute;

#[doc(hidden)]
pub use doc_comment::doc_comment;

#[doc(hidden)]
pub use safe_transmute::{transmute_one, TriviallyTransmutable};

/// TODO
#[macro_export(local_inner_macros)]
macro_rules! bitmask {

    // U8
    (
        $(#[$doc:meta])*
        $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $(#[$flag_doc:meta])*
        $flag:ident u8 {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {
        __bitmask_unchecked!( $(#[$doc])* $( [$( $derive ),*] )? $( ($access) )? $name $(#[$flag_doc])* $flag [u8] [8] { $( $(#[$member_doc])* $idx : $field )* });
    };

    // U16
    (
        $(#[$doc:meta])*
         $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $(#[$flag_doc:meta])*
        $flag:ident u16 {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {
        __bitmask_unchecked!( $(#[$doc])* $( [$( $derive ),*] )? $( ($access) )? $name $(#[$flag_doc])* $flag [u16] [16] { $( $(#[$member_doc])* $idx : $field )* });
    };

    // U32
    (
        $(#[$doc:meta])*
         $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $(#[$flag_doc:meta])*
        $flag:ident u32 {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {
        __bitmask_unchecked!( $(#[$doc])* $( [$( $derive ),*] )? $( ($access) )? $name $(#[$flag_doc])* $flag [u32] [32] { $( $(#[$member_doc])* $idx : $field )* });
    };

    // U64
    (
        $(#[$doc:meta])*
         $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $(#[$flag_doc:meta])*
        $flag:ident u64 {
            $( $(#[$member_doc:meta])* $idx:literal : $field:ident )*
        }
    ) => {
        __bitmask_unchecked!( $(#[$doc])* $( [$( $derive ),*] )? $( ($access) )? $name $(#[$flag_doc])* $flag [u64] [64] { $( $(#[$member_doc])* $idx : $field )* });
    };

    // U128
    (
        $(#[$doc:meta])*
         $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $(#[$flag_doc:meta])*
        $flag:ident u128 {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {
        __bitmask_unchecked!( $(#[$doc])* $( [$( $derive ),*] )? $( ($access) )? $name $(#[$flag_doc])* $flag [u128] [128] { $( $(#[$member_doc])* $idx : $field )* });
    };

    // USIZE
    (
        $(#[$doc:meta])*
         $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $(#[$flag_doc:meta])*
        $flag:ident usize {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {
        __bitmask_unchecked!( $(#[$doc])* $( [$( $derive ),*] )? $( ($access) )? $name $(#[$flag_doc])* $flag [usize] [usize::MAX] { $( $(#[$member_doc])* $idx : $field )* });
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __bitmask_unchecked {
    (
        $(#[$mask_doc:meta])*
        $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $(#[$flag_doc:meta])*
        $flag:ident [$type:ty] [$($bits:tt)*] {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {

        $(#[$flag_doc])*
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[repr(u8)]
        $( $access )? enum $flag {
            $( $(#[$member_doc])* $field = $idx ),*
        }

        impl std::ops::BitAnd for $flag {
            type Output = $name;
            fn bitand(self, rhs: Self) -> Self::Output {
                $name(0).set(self) & rhs
            }
        }

        impl std::ops::BitOr for $flag {
            type Output = $name;
            fn bitor(self, rhs: Self) -> Self::Output {
                $name(0).set(self) | rhs
            }
        }

        impl std::ops::BitXor for $flag {
            type Output = $name;
            fn bitxor(self, rhs: Self) -> Self::Output {
                $name(0).set(self) ^ rhs
            }
        }

        impl From<u8> for $flag {
            fn from(value: u8) -> $flag {
                $crate::transmute_one::<$flag>(&[value]).expect("") // TODO
            }
        }

        impl From<$flag> for u8 {
            fn from(value: $flag) -> u8 {
                value as u8
            }
        }

        unsafe impl TriviallyTransmutable for $flag {}

        $(#[$mask_doc])*
        $( #[derive($($derive:ident ),*)] )?
        $( $access )? struct $name($type);

        $crate::doc_comment! {
            core::concat!(
                "Constant values describing [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::doc_comment! {
                    core::concat!(
                      "Number of bits in an instance of [`", core::stringify!($name), "`]."
                    ),
                    pub const BITS: usize = $($bits)*;
                }
                $crate::doc_comment! {
                    core::concat!(
                      "Number of bytes used by an instance of [`", core::stringify!($name), "`]."
                    ),
                    pub const BYTES: usize = $($bits)* / 8;
                }
            }
        }

        $crate::doc_comment! {
            core::concat!(
                "Constructors for creating instances of [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::doc_comment! {
                    core::concat!(
                      "Create a new instance of [`", core::stringify!($name), "`] ",
                      "from a [`", core::stringify!($type), "`] value."
                    ),
                    pub fn new(value: $type) -> Self {
                        Self(value)
                    }
                }
            }
        }

        $crate::doc_comment! {
            core::concat!(
                "Current state of this bitmask."
            ),
            impl $name {
                $crate::doc_comment! {
                    core::concat!(
                      "Returns the current mask value as a [`", core::stringify!($type), "`]"
                    ),
                    pub fn value(&self) -> $type {
                        self.0
                    }
                }
            }
        }

        $crate::doc_comment! {
            core::concat!(
                "Field accessors by index for [`", core::stringify!($name), "`]."
            ),
            impl $name {
                    $crate::doc_comment! {
                    core::concat!(
                        "Returns the value of the bit at the supplied index as a boolean."
                    ),
                    pub fn get_index(&self, index: u8) -> bool {
                        ((self.0 >> index) & 1) == 1
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied index to `1`."
                    ),
                    pub fn set_index(&mut self, index: u8) {
                        self.0 |= (1 << index);
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied index to `0`."
                    ),
                    pub fn clear_index(&mut self, index: u8) {
                        self.0 &= (1 << index);
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Flips the value of the bit at the supplied index."
                    ),
                    pub fn toggle_index(&mut self, index: u8) {
                        self.0 ^= (1 << index);
                    }
                }
            }
        }

        $crate::doc_comment! {
            core::concat!(
                "Named field accessors by [`", core::stringify!($flag), "`] for [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::doc_comment! {
                    core::concat!(
                        "Returns the value of the bit at the supplied flag as a boolean."
                    ),
                    pub fn get(&self, flag: $flag) -> bool {
                        ((self.0 >> (flag as $type)) & 1) == 1
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied flag to `1`."
                    ),
                    pub fn set(&mut self, flag: $flag) -> &mut Self {
                        self.0 |= (1 << (flag as $type));
                        self
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied flag to `0`."
                    ),
                    pub fn clear(&mut self, flag: $flag) -> &mut Self {
                        self.0 &= (1 << (flag as $type));
                        self
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Flips the value of the bit at the supplied flag."
                    ),
                    pub fn toggle(&mut self, flag: $flag) -> &mut Self {
                        self.0 ^= (1 << (flag as $type));
                        self
                    }
                }
            }
        }

        $crate::doc_comment! {
            core::concat!(
                "Combinators for [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that do not match. ",
                        "Does not consume `self`."
                    ),
                    pub fn diff(&self, other: Self) -> Self {
                        Self(self.0 ^ other.0)
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that were set on either input. ",
                        "Does not consume `self`."
                    ),
                    pub fn combine(&self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that were set on both inputs. ",
                        "Does not consume `self`."
                    ),
                    pub fn intersect(&self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
            }
        }

        $crate::doc_comment! {
            core::concat!(
                "Conversion methods."
            ),
            impl $name {
                $crate::doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that do not match. ",
                        "Consumes `self`."
                    ),
                    pub fn into_diff(self, other: Self) -> Self {
                        Self(self.0 ^ other.0)
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that were set on either input. ",
                        "Consumes `self`."
                    ),
                    pub fn into_combined(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }

                $crate::doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that were set on both inputs. ",
                        "Consumes `self`."
                    ),
                    pub fn into_intersection(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> $name {
                $name(value)
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl std::ops::BitAnd<$flag> for $name {
            type Output = $name;
            fn bitand(self, rhs: $flag) -> Self::Output {
                $name(self.0  & (rhs as $type))
            }
        }

        impl std::ops::BitOr<$flag> for $name {
            type Output = $name;
            fn bitor(self, rhs: $flag) -> Self::Output {
                $name(self.0 | (rhs as $type))
            }
        }

        impl std::ops::BitXor<$flag> for $name {
            type Output = $name;
            fn bitxor(self, rhs: $flag) -> Self::Output {
                $name(self.0 ^ (rhs as $type))
            }
        }

        impl std::ops::BitAnd<$flag> for &$name {
            type Output = $name;
            fn bitand(self, rhs: $flag) -> Self::Output {
                $name(self.0  & (rhs as $type))
            }
        }

        impl std::ops::BitOr<$flag> for &$name {
            type Output = $name;
            fn bitor(self, rhs: $flag) -> Self::Output {
                $name(self.0 | (rhs as $type))
            }
        }

        impl std::ops::BitXor<$flag> for &$name {
            type Output = $name;
            fn bitxor(self, rhs: $flag) -> Self::Output {
                $name(self.0 ^ (rhs as $type))
            }
        }

        impl std::ops::BitAnd<$flag> for &mut $name {
            type Output = $name;
            fn bitand(self, rhs: $flag) -> Self::Output {
                $name(self.0  & (rhs as $type))
            }
        }

        impl std::ops::BitOr<$flag> for &mut $name {
            type Output = $name;
            fn bitor(self, rhs: $flag) -> Self::Output {
                $name(self.0 | (rhs as $type))
            }
        }

        impl std::ops::BitXor<$flag> for &mut $name {
            type Output = $name;
            fn bitxor(self, rhs: $flag) -> Self::Output {
                $name(self.0 ^ (rhs as $type))
            }
        }

        impl std::ops::BitAndAssign<$flag> for $name {
            fn bitand_assign(&mut self, rhs: $flag) {
                self.clear(rhs);
            }
        }

        impl std::ops::BitOrAssign<$flag> for $name {
            fn bitor_assign(&mut self, rhs: $flag) {
                self.set(rhs);
            }
        }

        impl std::ops::BitXorAssign<$flag> for $name {
            fn bitxor_assign(&mut self, rhs: $flag) {
                self.toggle(rhs);
            }
        }

        impl std::ops::BitAndAssign<$flag> for &mut $name {
            fn bitand_assign(&mut self, rhs: $flag) {
                self.clear(rhs);
            }
        }

        impl std::ops::BitOrAssign<$flag> for &mut $name {
            fn bitor_assign(&mut self, rhs: $flag) {
                self.set(rhs);
            }
        }

        impl std::ops::BitXorAssign<$flag> for &mut $name {
            fn bitxor_assign(&mut self, rhs: $flag) {
                self.toggle(rhs);
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                core::write!(f, "{}({:b})", core::stringify!($name), self)
            }
        }

        impl std::fmt::Binary for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                core::write!(f, "{:b}", self.0)
            }
        }
    };
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[cfg(doc)]
pub mod example {
    use super::*;

    bitmask! {
        /// TODO some description
        (pub) ExampleMask
        /// TODO some description
        ExampleFlags u8 {
            /// TODO some description
            0 : Flag0
            /// TODO some description
            1 : Flag1
            /// TODO some description
            2 : Flag2
            /// TODO some description
            3 : Flag3
            /// TODO some description
            4 : Flag4
            /// TODO some description
            5 : Flag5
            /// TODO some description
            6 : Flag6
            /// TODO some description
            7 : Flag7
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[cfg(test)]
mod test {
    use super::*;

    macro_rules! tests {
        ( $( $name:ident => $block:block )+ ) => {
            $(
                #[test]
                fn $name() $block
            )+
        };
    }

    bitmask! {
        /// My Test Thing
        (pub) MyMaskU8 MyFlagsU8 u8 {
            0 : Flag0 1 : Flag1 2 : Flag2 3 : Flag3 4 : Flag4 5 : Flag5 6 : Flag6 7 : Flag7
        }
    }

    bitmask! {
        /// My Test Thing
        (pub) MyMaskU16 MyFlagsU16 u16 {
            0 : Flag0 1 : Flag1 2 : Flag2 3 : Flag3 4 : Flag4 5 : Flag5 6 : Flag6 7 : Flag7
            8 : Flag8 9 : Flag9 10 : Flag10 11 : Flag11 12 : Flag12 13 : Flag13 14 : Flag14 15 : Flag15
        }
    }

    bitmask! {
        /// My Test Thing
        (pub) MyMaskU32 MyFlagsU32 u32 {
            0 : Flag0 1 : Flag1 2 : Flag2 3 : Flag3 4 : Flag4 5 : Flag5 6 : Flag6 7 : Flag7
            8 : Flag8 9 : Flag9 10 : Flag10 11 : Flag11 12 : Flag12 13 : Flag13 14 : Flag14 15 : Flag15
            16 : Flag16 17 : Flag17 18 : Flag18 19 : Flag19 20 : Flag20 21 : Flag21 22 : Flag22 23 : Flag23
            24 : Flag24 25 : Flag25 26 : Flag26 27 : Flag27 28 : Flag28 29 : Flag29 30 : Flag30 31 : Flag31
        }
    }

    bitmask! {
        /// My Test Thing
        (pub) MyMaskU64 MyFlagsU64 u64 {
            0 : Flag0 1 : Flag1 2 : Flag2 3 : Flag3 4 : Flag4 5 : Flag5 6 : Flag6 7 : Flag7
            8 : Flag8 9 : Flag9 10 : Flag10 11 : Flag11 12 : Flag12 13 : Flag13 14 : Flag14 15 : Flag15
            16 : Flag16 17 : Flag17 18 : Flag18 19 : Flag19 20 : Flag20 21 : Flag21 22 : Flag22 23 : Flag23
            24 : Flag24 25 : Flag25 26 : Flag26 27 : Flag27 28 : Flag28 29 : Flag29 30 : Flag30 31 : Flag31
            32 : Flag32 33 : Flag33 34 : Flag34 35 : Flag35 36 : Flag36 37 : Flag37 38 : Flag38 39 : Flag39
            40 : Flag40 41 : Flag41 42 : Flag42 43 : Flag43 44 : Flag44 45 : Flag45 46 : Flag46 47 : Flag47
            48 : Flag48 49 : Flag49 50 : Flag50 51 : Flag51 52 : Flag52 53 : Flag53 54 : Flag54 55 : Flag55
            56 : Flag56 57 : Flag57 58 : Flag58 59 : Flag59 60 : Flag60 61 : Flag61 62 : Flag62 63 : Flag63
        }
    }

    bitmask! {
        /// My Test Thing
        (pub) MyMaskU128 MyFlagsU128 u128 {
            0 : Flag0 1 : Flag1 2 : Flag2 3 : Flag3 4 : Flag4 5 : Flag5 6 : Flag6 7 : Flag7
            8 : Flag8 9 : Flag9 10 : Flag10 11 : Flag11 12 : Flag12 13 : Flag13 14 : Flag14 15 : Flag15
            16 : Flag16 17 : Flag17 18 : Flag18 19 : Flag19 20 : Flag20 21 : Flag21 22 : Flag22 23 : Flag23
            24 : Flag24 25 : Flag25 26 : Flag26 27 : Flag27 28 : Flag28 29 : Flag29 30 : Flag30 31 : Flag31
            32 : Flag32 33 : Flag33 34 : Flag34 35 : Flag35 36 : Flag36 37 : Flag37 38 : Flag38 39 : Flag39
            40 : Flag40 41 : Flag41 42 : Flag42 43 : Flag43 44 : Flag44 45 : Flag45 46 : Flag46 47 : Flag47
            48 : Flag48 49 : Flag49 50 : Flag50 51 : Flag51 52 : Flag52 53 : Flag53 54 : Flag54 55 : Flag55
            56 : Flag56 57 : Flag57 58 : Flag58 59 : Flag59 60 : Flag60 61 : Flag61 62 : Flag62 63 : Flag63
            64 : Flag64 65 : Flag65 66 : Flag66 67 : Flag67 68 : Flag68 69 : Flag69 70 : Flag70 71 : Flag71
            72 : Flag72 73 : Flag73 74 : Flag74 75 : Flag75 76 : Flag76 77 : Flag77 78 : Flag78 79 : Flag79
            80 : Flag80 81 : Flag81 82 : Flag82 83 : Flag83 84 : Flag84 85 : Flag85 86 : Flag86 87 : Flag87
            88 : Flag88 89 : Flag89 90 : Flag90 91 : Flag91 92 : Flag92 93 : Flag93 94 : Flag94 95 : Flag95
            96 : Flag96 97 : Flag97 98 : Flag98 99 : Flag99 100 : Flag100 101 : Flag101 102 : Flag102 103 : Flag103
            104 : Flag104 105 : Flag105 106 : Flag106 107 : Flag107 108 : Flag108 109 : Flag109 110 : Flag110 111 : Flag111
            112 : Flag112 113 : Flag113 114 : Flag114 115 : Flag115 116 : Flag116 117 : Flag117 118 : Flag118 119 : Flag119
            120 : Flag120 121 : Flag121 122 : Flag122 123 : Flag123 124 : Flag124 125 : Flag125 126 : Flag126 127 : Flag127
        }
    }

    // constructors
    tests! {
        new_u8 => {
            let mask = MyMaskU8(123);
            assert_eq!(123, mask.value())
        }
        new_u16 => {
            let mask = MyMaskU16(123);
            assert_eq!(123, mask.value())
        }
        new_u32 => {
            let mask = MyMaskU32(123);
            assert_eq!(123, mask.value())
        }
        new_u64 => {
            let mask = MyMaskU64(123);
            assert_eq!(123, mask.value())
        }
        new_u128 => {
            let mask = MyMaskU128(123);
            assert_eq!(123, mask.value())
        }
    }

    // defaults
    tests! {
        default_u8 => {
            assert_eq!(0, MyMaskU8::default().value())
        }
        default_u16 => {
            assert_eq!(0, MyMaskU16::default().value())
        }
        default_u32 => {
            assert_eq!(0, MyMaskU32::default().value())
        }
        default_u64 => {
            assert_eq!(0, MyMaskU64::default().value())
        }
        default_u128 => {
            assert_eq!(0, MyMaskU128::default().value())
        }
    }

    // bit count
    tests! {
        bit_count_u8 => {
            let mask = MyMaskU8(123);
            assert_eq!(8, MyMaskU8::BITS)
        }
        bit_count_u16 => {
            let mask = MyMaskU16(123);
            assert_eq!(16, MyMaskU16::BITS)
        }
        bit_count_u32 => {
            let mask = MyMaskU32(123);
            assert_eq!(32, MyMaskU32::BITS)
        }
        bit_count_u64 => {
            let mask = MyMaskU64(123);
            assert_eq!(64, MyMaskU64::BITS)
        }
        bit_count_u128 => {
            let mask = MyMaskU128(123);
            assert_eq!(128, MyMaskU128::BITS)
        }
    }

    // byte count
    tests! {
        byte_count_u8 => {
            let mask = MyMaskU8(123);
            assert_eq!(1, MyMaskU8::BYTES)
        }
        byte_count_u16 => {
            let mask = MyMaskU16(123);
            assert_eq!(2, MyMaskU16::BYTES)
        }
        byte_count_u32 => {
            let mask = MyMaskU32(123);
            assert_eq!(4, MyMaskU32::BYTES)
        }
        byte_count_u64 => {
            let mask = MyMaskU64(123);
            assert_eq!(8, MyMaskU64::BYTES)
        }
        byte_count_u128 => {
            let mask = MyMaskU128(123);
            assert_eq!(16, MyMaskU128::BYTES)
        }
    }
}
