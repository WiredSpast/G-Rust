use std::fmt::{Debug, Display, Formatter};
use std::ops::*;
use crate::misc::hclient::{HClient, CUR_CLIENT};
use super::packetvariable::PacketVariable;

/**
 * Behaves like an i32 when connected to Flash or Nitro <br>
 * Behaves like an i64 when connected to Unity
 */
#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct LegacyId(pub i64);

/**
 * Behaves like an i32 when connected to Flash or Nitro <br>
 * Behaves like an i16 when connected to Unity
 */
#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct LegacyLength(pub i32);

/**
 * Always behaves like an i64 <br>
 * Reads a string and parses it to i64 when connected to Flash or Nitro <br>
 * Reads an i64 when connected to Unity
 */
#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct LegacyStringId(pub i64);

/**
 * Always behaves like an f64 <br>
 * Reads a string and parses it to f64 when connected to Flash or Nitro <br>
 * Reads an f64 when connected to Unity
 */
#[derive(PartialEq, Clone, Copy, Default)]
pub struct LegacyDouble(pub f64);

macro_rules! impl_op {
    ($legacy_name:ident, $legacy_ty:ident => $($op_name:ident, $op_ass_name:ident, $function_name:ident, $function_ass_name:ident, $op:expr, $($ty:ident) +); +;) => ($(
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
    ($($name:ident, $ty_max:ident, $ty_flash:ident, $ty_unity:ident); +;) => ($(
        impl Deref for $name {
            type Target = $ty_max;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-(self.0))
            }
        }

        impl_op! {
            $name, $ty_max =>
                Add, AddAssign, add, add_assign, | a, b | a + b, $ty_flash $ty_unity;
                Div, DivAssign, div, div_assign, | a, b | a / b, $ty_flash $ty_unity;
                Mul, MulAssign, mul, mul_assign, | a, b | a * b, $ty_flash $ty_unity;
                Rem, RemAssign, rem, rem_assign, | a, b | a % b, $ty_flash $ty_unity;
                Sub, SubAssign, sub, sub_assign, | a, b | a - b, $ty_flash $ty_unity;
        }
    )+)
}

macro_rules! impl_legacy_bitwise {
    ($($name:ident, $ty_max:ident, $ty_flash:ident, $ty_unity:ident); +;) => ($(
        impl Not for $name {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl_op! {
            $name, $ty_max =>
               BitAnd, BitAndAssign, bitand, bitand_assign, | a, b | a & b, $ty_flash $ty_unity;
               BitOr, BitOrAssign, bitor, bitor_assign, | a, b | a | b, $ty_flash $ty_unity;
               BitXor, BitXorAssign, bitxor, bitxor_assign, | a, b | a ^ b, $ty_flash $ty_unity;
               Shl, ShlAssign, shl, shl_assign, | a, b | a << b, $ty_flash $ty_unity;
               Shr, ShrAssign, shr, shr_assign, | a, b | a >> b, $ty_flash $ty_unity;
        }
    )+)
}

impl_legacy! {
    LegacyId, i64, i32, i64;
    LegacyLength, i32, i32, i16;
    LegacyStringId, i64, i32, i64;
    LegacyDouble, f64, f32, f64;
}

impl_legacy_bitwise! {
    LegacyId, i64, i32, i64;
    LegacyLength, i32, i32, i16;
    LegacyStringId, i64, i32, i64;
}

impl PacketVariable for LegacyId {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (Self(i64::from_packet(bytes).0), 8)
        } else {
            (Self(i32::from_packet(bytes).0 as i64), 4)
        }
    }

    fn to_packet(&self) -> Vec<u8> {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (self.0).to_packet()
        } else {
            (self.0 as i32).to_packet()
        }
    }
}

impl PacketVariable for LegacyLength {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (Self(i16::from_packet(bytes).0 as i32), 8)
        } else {
            (Self(i32::from_packet(bytes).0), 4)
        }
    }

    fn to_packet(&self) -> Vec<u8> {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (self.0 as i16).to_packet()
        } else {
            (self.0).to_packet()
        }
    }
}

impl PacketVariable for LegacyStringId {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (Self(i64::from_packet(bytes).0), 8)
        } else {
            let (s, size) = String::from_packet(bytes);
            (Self(s.parse::<i64>().unwrap()), size)
        }
    }

    fn to_packet(&self) -> Vec<u8> {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (self.0).to_packet()
        } else {
            let res = (self.0.to_string()).to_packet();
            res
        }
    }
}

impl PacketVariable for LegacyDouble {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (Self(f64::from_packet(bytes).0), 8)
        } else {
            let (s, size) = String::from_packet(bytes);
            (Self(s.parse::<f64>().unwrap()), size)
        }
    }

    fn to_packet(&self) -> Vec<u8> {
        if *CUR_CLIENT.lock().unwrap() == HClient::Unity {
            (self.0).to_packet()
        } else {
            let res = (self.0.to_string()).to_packet();
            res
        }
    }
}
