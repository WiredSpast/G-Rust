use std::fmt::{Debug, Display, Formatter};
use std::ops::*;
use crate::protocol::hotel::{Hotel, CUR_HOTEL};
use super::packetvariable::PacketVariable;

pub struct LegacyId(pub i64);
pub struct LegacyLength(pub i32);

macro_rules! impl_op {
    ($legacy_name:ident, $legacy_ty:ident => $($op_name:ident, $op_ass_name:ident, $function_name:ident, $function_ass_name:ident, $op:expr, $($ty:ident) +); +) => ($(
        impl $op_name for $legacy_name {
            type Output = Self;

            fn $function_name(self, rhs: Self) -> Self::Output {
                Self(($op)(self.0, rhs.0))
            }
        }

        impl $op_ass_name for $legacy_name {
            fn $function_ass_name(&mut self, rhs: Self) {
                *self = Self(($op)(self.0, rhs.0))
            }
        }

        $(
            impl $op_name<$ty> for $legacy_name {
                type Output = Self;

                fn $function_name(self, rhs: $ty) -> Self::Output {
                    Self(($op)(self.0, rhs as $legacy_ty))
                }
            }

            impl $op_ass_name<$ty> for $legacy_name {
                fn $function_ass_name(&mut self, rhs: $ty) {
                    *self = Self(($op)(self.0, rhs as $legacy_ty))
                }
            }
        )+
    )+)
}

macro_rules! impl_legacy {
    ($($legacy_name:ident $legacy_ty_max:ident $legacy_ty_flash:ident $legacy_ty_unity:ident); +) => ($(
        impl Deref for $legacy_name {
            type Target = $legacy_ty_max;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $legacy_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Debug for $legacy_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Display for $legacy_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Neg for $legacy_name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-(self.0))
            }
        }

        impl Not for $legacy_name {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl PacketVariable for $legacy_name {
            fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
                if *CUR_HOTEL.lock().unwrap() == Hotel::Unity {
                    (Self($legacy_ty_unity::from_packet(bytes).0 as $legacy_ty_max), 8)
                } else {
                    (Self($legacy_ty_flash::from_packet(bytes).0 as $legacy_ty_max), 4)
                }
            }

            fn to_packet(&self) -> Vec<u8> {
                if *CUR_HOTEL.lock().unwrap() == Hotel::Unity {
                    (self.0 as $legacy_ty_unity).to_packet()
                } else {
                    (self.0 as $legacy_ty_flash).to_packet()
                }
            }
        }

        impl_op! {
            $legacy_name, $legacy_ty_max =>
                Add, AddAssign, add, add_assign, | a, b | a + b, $legacy_ty_flash $legacy_ty_unity;
                BitAnd, BitAndAssign, bitand, bitand_assign, | a, b | a & b, $legacy_ty_flash $legacy_ty_unity;
                BitOr, BitOrAssign, bitor, bitor_assign, | a, b | a | b, $legacy_ty_flash $legacy_ty_unity;
                BitXor, BitXorAssign, bitxor, bitxor_assign, | a, b | a ^ b, $legacy_ty_flash $legacy_ty_unity;
                Div, DivAssign, div, div_assign, | a, b | a / b, $legacy_ty_flash $legacy_ty_unity;
                Mul, MulAssign, mul, mul_assign, | a, b | a * b, $legacy_ty_flash $legacy_ty_unity;
                Rem, RemAssign, rem, rem_assign, | a, b | a % b, $legacy_ty_flash $legacy_ty_unity;
                Shl, ShlAssign, shl, shl_assign, | a, b | a << b, $legacy_ty_flash $legacy_ty_unity;
                Shr, ShrAssign, shr, shr_assign, | a, b | a >> b, $legacy_ty_flash $legacy_ty_unity;
                Sub, SubAssign, sub, sub_assign, | a, b | a - b, $legacy_ty_flash $legacy_ty_unity
        }
    )+)
}

impl_legacy! {
    LegacyId i64 i32 i64;
    LegacyLength i32 i32 i16
}