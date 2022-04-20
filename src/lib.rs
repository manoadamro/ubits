//! TODO

// -------------------------------------------------------------------------------------------------
// Imports

extern crate doc_comment;
extern crate safe_transmute;

#[doc(hidden)]
pub use doc_comment::doc_comment as __doc_comment;

#[doc(hidden)]
pub use safe_transmute::{
    transmute_one as __transmute_one,
    TriviallyTransmutable as __TriviallyTransmutable
};

// -------------------------------------------------------------------------------------------------
// Generator Macros

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

        __def_flag_enum! {
            $(#[$flag_doc])*
            $( ($access) )? $name
            $flag {
                $(
                    $(#[$member_doc])*
                    $idx : $field
                )*
            }
        }

        __def_mask_struct! {
            $(#[$mask_doc])*
            $( [$( $derive ),*] )?
            $( ($access) )? $name
            $flag [$type] [$($bits)*] {
                $(
                    $(#[$member_doc])*
                    $idx : $field
                )*
            }
        }
    };
}

// -------------------------------------------------------------------------------------------------
// Type Definitions

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __def_flag_enum {
    (
        $(#[$flag_doc:meta])*
        $( ($access:vis) )? $name:ident
        $flag:ident {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {

        // Enum: $flag
        $(#[$flag_doc])*
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[repr(u8)]
        $( $access )? enum $flag {
            $( $(#[$member_doc])* $field = $idx ),*
        }

        // Bitwise: $flag + $flag
        __impl_bitwise_operators! {
            [Self] for $flag : (self rhs -> $name)
            BitAnd => { $name(0).set(self) & rhs }
            BitOr => { $name(0).set(self) | rhs }
            BitXor => { $name(0).set(self) ^ rhs }
        }

        // Bitwise: u8 -> $flag
        __impl_from! { u8 as $flag (value) => {
            $crate::__transmute_one::<$flag>(&[value]).expect("") // TODO
        }}

        // Bitwise: $flag -> u8
        __impl_from! { $flag as u8 (value) => {
            value as u8
        }}

        unsafe impl __TriviallyTransmutable for $flag {}
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __def_mask_struct {
    (
        $(#[$mask_doc:meta])*
        $( [$( $derive:ident ),*] )?
        $( ($access:vis) )? $name:ident
        $flag:ident [$type:ty] [$($bits:tt)*] {
            $(
                $(#[$member_doc:meta])*
                $idx:literal : $field:ident
            )*
        }
    ) => {

        // Struct: $name
        $(#[$mask_doc])*
        $( #[derive($($derive:ident ),*)] )?
        $( $access )? struct $name($type);

        // Constants
        __impl_mask_constants! {
            $name { bits: [$($bits)*] }
        }

        // Constructors
        __impl_mask_ctors! {
            $name : $type
        }

        // State
        __impl_mask_state! {
            $name : $type
        }

        // Indexers
        __impl_mask_index_accessors! {
            $name
        }

        // Flags
        __impl_mask_flag_accessors! {
            $name $flag $type
        }

        // Combinators
        __impl_mask_flag_combinators! {
            $name
        }

        // Converters
        __impl_mask_flag_converters! {
            $name
        }

        // Name: $name
        __impl_default! {
            $name => { $name(0) }
        }

        // From: $type -> $name
        __impl_from! {
            $type as $name (value) => {
                $name(value)
            }
        }

        // Bitwise: $name + $name
        __impl_bitwise_operators! {
            [Self] for $name : (self rhs -> Self)
            BitAnd => { Self(self.0 & rhs.0) }
            BitOr => { Self(self.0 | rhs.0) }
            BitXor => {  Self(self.0 ^ rhs.0) }
            BitAndAssign => { self.0 &= rhs.0; }
            BitOrAssign => { self.0 |= rhs.0; }
            BitXorAssign => { self.0 ^= rhs.0; }
        }

        // Bitwise: $name + $flag
        __impl_bitwise_operators! {
            [$flag] for $name : (self rhs -> $name)
            BitAnd => { $name(self.0  & (rhs as $type)) }
            BitOr => { $name(self.0 | (rhs as $type)) }
            BitXor => { $name(self.0 ^ (rhs as $type)) }
            BitAndAssign => { self.clear(rhs); }
            BitOrAssign => { self.set(rhs); }
            BitXorAssign => { self.toggle(rhs); }
        }

        // Bitwise: &$name + $flag
        __impl_bitwise_operators! {
            [$flag] for & $name : (self rhs -> $name)
            BitAnd => { $name(self.0  & (rhs as $type)) }
            BitOr => { $name(self.0 | (rhs as $type)) }
            BitXor => { $name(self.0 ^ (rhs as $type)) }
        }

        // Bitwise: $mut $name + $flag
        __impl_bitwise_operators! {
            [$flag] for &mut $name : (self rhs -> $name)
            BitAnd => { $name(self.0  & (rhs as $type)) }
            BitOr => { $name(self.0 | (rhs as $type)) }
            BitXor => { $name(self.0 ^ (rhs as $type)) }
        }

        // Bitwise: Debug & Binary
        __impl_formatters! {
            $name (self f) {
                Debug => { core::write!(f, "{}({:b})", core::stringify!($name), self) }
                Binary => { core::write!(f, "{:b}", self.0) }
            }
        }
    };
}

// -------------------------------------------------------------------------------------------------
// Mask Implementations

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mask_constants {
    ($name:ident { bits: [$($bits:tt)*] }) => {
        $crate::__doc_comment! {
            core::concat!(
                "Constant values describing [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::__doc_comment! {
                    core::concat!(
                      "Number of bits in an instance of [`", core::stringify!($name), "`]."
                    ),
                    pub const BITS: usize = $($bits)*;
                }
                $crate::__doc_comment! {
                    core::concat!(
                      "Number of bytes used by an instance of [`", core::stringify!($name), "`]."
                    ),
                    pub const BYTES: usize = $($bits)* / 8;
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mask_ctors {
    ( $name:ident : $type:ty ) => {
        $crate::__doc_comment! {
            core::concat!(
                "Constructors for creating instances of [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::__doc_comment! {
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mask_state {
    ($name:ident : $type:ty) => {
        $crate::__doc_comment! {
            core::concat!(
                "Current state of this bitmask."
            ),
            impl $name {
                $crate::__doc_comment! {
                    core::concat!(
                      "Returns the current mask value as a [`", core::stringify!($type), "`]"
                    ),
                    pub fn value(&self) -> $type {
                        self.0
                    }
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mask_index_accessors {
    ($name:ident) => {
        $crate::__doc_comment! {
            core::concat!(
                "Field accessors by index for [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::__doc_comment! {
                    core::concat!(
                        "Returns the value of the bit at the supplied index as a boolean."
                    ),
                    pub fn get_index(&self, index: u8) -> bool {
                        ((self.0 >> index) & 1) == 1
                    }
                }

                $crate::__doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied index to `1`."
                    ),
                    pub fn set_index(&mut self, index: u8) {
                        self.0 |= (1 << index);
                    }
                }

                $crate::__doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied index to `0`."
                    ),
                    pub fn clear_index(&mut self, index: u8) {
                        self.0 &= (1 << index);
                    }
                }

                $crate::__doc_comment! {
                    core::concat!(
                        "Flips the value of the bit at the supplied index."
                    ),
                    pub fn toggle_index(&mut self, index: u8) {
                        self.0 ^= (1 << index);
                    }
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mask_flag_accessors {
    ($name:ident $flag:ident $type:ty) => {
        $crate::__doc_comment! {
            core::concat!(
                "Named field accessors by [`", core::stringify!($flag), "`] for [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::__doc_comment! {
                    core::concat!(
                        "Returns the value of the bit at the supplied flag as a boolean."
                    ),
                    pub fn get(&self, flag: $flag) -> bool {
                        ((self.0 >> (flag as $type)) & 1) == 1
                    }
                }

                $crate::__doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied flag to `1`."
                    ),
                    pub fn set(&mut self, flag: $flag) -> &mut Self {
                        self.0 |= (1 << (flag as $type));
                        self
                    }
                }

                $crate::__doc_comment! {
                    core::concat!(
                        "Sets the value of the bit at the supplied flag to `0`."
                    ),
                    pub fn clear(&mut self, flag: $flag) -> &mut Self {
                        self.0 &= (1 << (flag as $type));
                        self
                    }
                }

                $crate::__doc_comment! {
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mask_flag_combinators {
    ($name:ident) => {
        $crate::__doc_comment! {
            core::concat!(
                "Combinators for [`", core::stringify!($name), "`]."
            ),
            impl $name {
                $crate::__doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that do not match. ",
                        "Does not consume `self`."
                    ),
                    pub fn diff(&self, other: Self) -> Self {
                        Self(self.0 ^ other.0)
                    }
                }

                $crate::__doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that were set on either input. ",
                        "Does not consume `self`."
                    ),
                    pub fn combine(&self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }

                $crate::__doc_comment! {
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mask_flag_converters {
    ($name:ident) => {
        $crate::__doc_comment! {
            core::concat!(
                "Conversion methods."
            ),
            impl $name {
                $crate::__doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that do not match. ",
                        "Consumes `self`."
                    ),
                    pub fn into_diff(self, other: Self) -> Self {
                        Self(self.0 ^ other.0)
                    }
                }

                $crate::__doc_comment! {
                    core::concat!(
                        "Returns a new [`", core::stringify!($name), "`]",
                        "with ones for flags that were set on either input. ",
                        "Consumes `self`."
                    ),
                    pub fn into_combined(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }

                $crate::__doc_comment! {
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
    };
}

// -------------------------------------------------------------------------------------------------
// Trait Implementations

// Operators

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __impl_bitwise_operators {

    // Owned
    (
        [$flag:ty] for $dest:ident : ($self:ident $other:ident -> $output:ident)
        $( BitAnd => $bitand:block )?
        $( BitOr => $bitor:block )?
        $( BitXor => $bitxor:block )?
        $( BitAndAssign => $bitand_assign:block )?
        $( BitOrAssign => $bitor_assign:block )?
        $( BitXorAssign => $bitxor_assign:block )?
    ) => {
        $( __impl_operator! {BitAnd [$flag] bitand for $dest ($self $other -> $output) => $bitand} )?
        $( __impl_operator! {BitOr [$flag] bitor for $dest ($self $other -> $output) => $bitor} )?
        $( __impl_operator! {BitXor [$flag] bitxor for $dest ($self $other -> $output) => $bitxor} )?

        $( __impl_assign_operator! {BitAndAssign [$flag] bitand_assign for $dest ($self $other) => $bitand_assign} )?
        $( __impl_assign_operator! {BitOrAssign [$flag] bitor_assign for $dest ($self $other) => $bitor_assign} )?
        $( __impl_assign_operator! {BitXorAssign [$flag] bitxor_assign for $dest ($self $other) => $bitxor_assign} )?
    };

    // Reference
    (
        [$flag:ty] for & $dest:ident : ($self:ident $other:ident -> $output:ident)
        $( BitAnd => $bitand:block )?
        $( BitOr => $bitor:block )?
        $( BitXor => $bitxor:block )?
    ) => {
        $( __impl_operator! {BitAnd [$flag] bitand for & $dest ($self $other -> $output) => $bitand} )?
        $( __impl_operator! {BitOr [$flag] bitor for & $dest ($self $other -> $output) => $bitor} )?
        $( __impl_operator! {BitXor [$flag] bitxor for & $dest ($self $other -> $output) => $bitxor} )?
    };

    // Mutable Reference
    (
        [$flag:ty] for &mut $dest:ident : ($self:ident $other:ident -> $output:ident)
        $( BitAnd => $bitand:block )?
        $( BitOr => $bitor:block )?
        $( BitXor => $bitxor:block )?
    ) => {
        $( __impl_operator! {BitAnd [$flag] bitand for &mut $dest ($self $other -> $output) => $bitand} )?
        $( __impl_operator! {BitOr [$flag] bitor for &mut $dest ($self $other -> $output) => $bitor} )?
        $( __impl_operator! {BitXor [$flag] bitxor for &mut $dest ($self $other -> $output) => $bitxor} )?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_operator {

    // Owned
    ( $op_type:ident [$rhs:ty] $op_lower:ident for $dest:ident ($self:ident $other:ident -> $output:ident) => $block:block ) => {
        impl std::ops::$op_type<$rhs> for $dest {
            type Output = $output;
            fn $op_lower($self, $other: $rhs) -> Self::Output $block
        }
    };

    // Reference
    ( $op_type:ident [$rhs:ty] $op_lower:ident for & $dest:ident ($self:ident $other:ident -> $output:ident) => $block:block ) => {
        impl std::ops::$op_type<$rhs> for &$dest {
            type Output = $output;
            fn $op_lower($self, $other: $rhs) -> Self::Output $block
        }
    };

    // Mutable Reference
    ( $op_type:ident [$rhs:ty] $op_lower:ident for &mut $dest:ident ($self:ident $other:ident -> $output:ident) => $block:block ) => {
        impl std::ops::$op_type<$rhs> for &mut $dest {
            type Output = $output;
            fn $op_lower($self, $other: $rhs) -> Self::Output $block
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_assign_operator {
    ( $op_type:ident [$rhs:ty] $op_lower:ident for $dest:ident ($self:ident $other:ident) => $block:block ) => {
        impl std::ops::$op_type<$rhs> for $dest {
            fn $op_lower(&mut $self, $other: $rhs) $block
        }
    };
}

// Defaults

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_default {
    ($name:ident => $block:block) => {
        impl Default for $name {
            fn default() -> Self $block
        }
    };
}

// Converters

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_from {
    ($from:ty as $to:ident ($arg:ident) => $block:block) => {
        impl From<$from> for $to {
            fn from($arg: $from) -> $to $block
        }
    };
}

// Formatters

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __impl_formatters {
    ($name:ident ($self:ident $f:ident) { $( $formatter:ident => $block:block )+ } ) => {
        $( __impl_formatter!($formatter for $name ($self $f) => $block); )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_formatter {
    ($formatter:ident for $name:ident ($self:ident $f:ident) => $block:block) => {
        impl std::fmt::$formatter for $name {
            fn fmt(&$self, $f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result $block
        }
    };
}

// -------------------------------------------------------------------------------------------------
// Example

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

// -------------------------------------------------------------------------------------------------
// Tests

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

    // integer value
    // TODO

    // binary value
    // TODO

    // get index
    // TODO

    // set index
    // TODO

    // clear index
    // TODO

    // toggle index
    // TODO

    // get
    // TODO

    // set
    // TODO

    // clear
    // TODO

    // toggle
    // TODO

    // diff
    // TODO

    // combine
    // TODO

    // intersect
    // TODO

    // into diff
    // TODO

    // into combine
    // TODO

    // into intersect
    // TODO

    // mask from flag
    // TODO

    // $name & $name
    // TODO

    // $name | $name
    // TODO

    // $name ^ $name
    // TODO

    // $name &= $name
    // TODO

    // $name |= $name
    // TODO

    // $name ^= $name
    // TODO

    // $name & $flag
    // TODO

    // $name | $flag
    // TODO

    // $name ^ $flag
    // TODO

    // $name &= $flag
    // TODO

    // $name |= $flag
    // TODO

    // $name ^= $flag
    // TODO

    // flag from u8
    // TODO

    // u8 from flag
    // TODO

    // $flag & $flag
    // TODO

    // $flag | $flag
    // TODO

    // $flag ^ $flag
    // TODO

    // binary format
    // TODO

    // debug format
    // TODO
}

// -------------------------------------------------------------------------------------------------
