// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_PARAMETER_TYPE: u8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_PARAMETER_TYPE: u8 = 6;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_PARAMETER_TYPE: [ParameterType; 7] = [
    ParameterType::hlint,
    ParameterType::hluint,
    ParameterType::hllong,
    ParameterType::hlulong,
    ParameterType::hlstring,
    ParameterType::hlbool,
    ParameterType::hlvecbytes,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ParameterType(pub u8);
#[allow(non_upper_case_globals)]
impl ParameterType {
    pub const hlint: Self = Self(0);
    pub const hluint: Self = Self(1);
    pub const hllong: Self = Self(2);
    pub const hlulong: Self = Self(3);
    pub const hlstring: Self = Self(4);
    pub const hlbool: Self = Self(5);
    pub const hlvecbytes: Self = Self(6);

    pub const ENUM_MIN: u8 = 0;
    pub const ENUM_MAX: u8 = 6;
    pub const ENUM_VALUES: &'static [Self] = &[
        Self::hlint,
        Self::hluint,
        Self::hllong,
        Self::hlulong,
        Self::hlstring,
        Self::hlbool,
        Self::hlvecbytes,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::hlint => Some("hlint"),
            Self::hluint => Some("hluint"),
            Self::hllong => Some("hllong"),
            Self::hlulong => Some("hlulong"),
            Self::hlstring => Some("hlstring"),
            Self::hlbool => Some("hlbool"),
            Self::hlvecbytes => Some("hlvecbytes"),
            _ => None,
        }
    }
}
impl core::fmt::Debug for ParameterType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl<'a> flatbuffers::Follow<'a> for ParameterType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
        Self(b)
    }
}

impl flatbuffers::Push for ParameterType {
    type Output = ParameterType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ParameterType {
    type Scalar = u8;
    #[inline]
    fn to_little_endian(self) -> u8 {
        self.0.to_le()
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(v: u8) -> Self {
        let b = u8::from_le(v);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for ParameterType {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        u8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for ParameterType {}
