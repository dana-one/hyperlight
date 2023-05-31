// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ERROR_CODE: u64 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ERROR_CODE: u64 = 16;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ERROR_CODE: [ErrorCode; 16] = [
  ErrorCode::NoError,
  ErrorCode::UnsupportedParameterType,
  ErrorCode::GuestFunctionNameNotProvided,
  ErrorCode::GuestFunctionNotFound,
  ErrorCode::GuestFunctionIncorrecNoOfParameters,
  ErrorCode::GispatchFunctionPointerNotSet,
  ErrorCode::OutbError,
  ErrorCode::UnknownError,
  ErrorCode::StackOverflow,
  ErrorCode::GsCheckFailed,
  ErrorCode::TooManyGuestFunctions,
  ErrorCode::FailureInDlmalloc,
  ErrorCode::MallocFailed,
  ErrorCode::GuestFunctionParameterTypeMismatch,
  ErrorCode::GuestError,
  ErrorCode::ArrayLengthParamIsMissing,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ErrorCode(pub u64);
#[allow(non_upper_case_globals)]
impl ErrorCode {
  pub const NoError: Self = Self(0);
  pub const UnsupportedParameterType: Self = Self(2);
  pub const GuestFunctionNameNotProvided: Self = Self(3);
  pub const GuestFunctionNotFound: Self = Self(4);
  pub const GuestFunctionIncorrecNoOfParameters: Self = Self(5);
  pub const GispatchFunctionPointerNotSet: Self = Self(6);
  pub const OutbError: Self = Self(7);
  pub const UnknownError: Self = Self(8);
  pub const StackOverflow: Self = Self(9);
  pub const GsCheckFailed: Self = Self(10);
  pub const TooManyGuestFunctions: Self = Self(11);
  pub const FailureInDlmalloc: Self = Self(12);
  pub const MallocFailed: Self = Self(13);
  pub const GuestFunctionParameterTypeMismatch: Self = Self(14);
  pub const GuestError: Self = Self(15);
  pub const ArrayLengthParamIsMissing: Self = Self(16);

  pub const ENUM_MIN: u64 = 0;
  pub const ENUM_MAX: u64 = 16;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NoError,
    Self::UnsupportedParameterType,
    Self::GuestFunctionNameNotProvided,
    Self::GuestFunctionNotFound,
    Self::GuestFunctionIncorrecNoOfParameters,
    Self::GispatchFunctionPointerNotSet,
    Self::OutbError,
    Self::UnknownError,
    Self::StackOverflow,
    Self::GsCheckFailed,
    Self::TooManyGuestFunctions,
    Self::FailureInDlmalloc,
    Self::MallocFailed,
    Self::GuestFunctionParameterTypeMismatch,
    Self::GuestError,
    Self::ArrayLengthParamIsMissing,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NoError => Some("NoError"),
      Self::UnsupportedParameterType => Some("UnsupportedParameterType"),
      Self::GuestFunctionNameNotProvided => Some("GuestFunctionNameNotProvided"),
      Self::GuestFunctionNotFound => Some("GuestFunctionNotFound"),
      Self::GuestFunctionIncorrecNoOfParameters => Some("GuestFunctionIncorrecNoOfParameters"),
      Self::GispatchFunctionPointerNotSet => Some("GispatchFunctionPointerNotSet"),
      Self::OutbError => Some("OutbError"),
      Self::UnknownError => Some("UnknownError"),
      Self::StackOverflow => Some("StackOverflow"),
      Self::GsCheckFailed => Some("GsCheckFailed"),
      Self::TooManyGuestFunctions => Some("TooManyGuestFunctions"),
      Self::FailureInDlmalloc => Some("FailureInDlmalloc"),
      Self::MallocFailed => Some("MallocFailed"),
      Self::GuestFunctionParameterTypeMismatch => Some("GuestFunctionParameterTypeMismatch"),
      Self::GuestError => Some("GuestError"),
      Self::ArrayLengthParamIsMissing => Some("ArrayLengthParamIsMissing"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ErrorCode {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ErrorCode {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u64>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ErrorCode {
    type Output = ErrorCode;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u64>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ErrorCode {
  type Scalar = u64;
  #[inline]
  fn to_little_endian(self) -> u64 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u64) -> Self {
    let b = u64::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ErrorCode {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u64::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ErrorCode {}
