use crate::{NullTerm, Seq, SeqMut};

/// C's `char` type that is known to not be `'\0'`.
pub type CChar = core::num::NonZero<core::ffi::c_char>;

/// Reference to a C string.
pub type CStr<'a> = Seq<'a, CChar, NullTerm>;

/// Mutable reference to a C string.
pub type CStrMut<'a> = SeqMut<'a, CChar, NullTerm>;

impl<'a> CStr<'a> {
    /// Casts from [`core::ffi::CStr`].
    pub const fn from_cstr(from: &'a core::ffi::CStr) -> Self {
        // SAFETY: `core::ffi::CStr` represents a valid C string, which is 
        // readable up to a null terminator
        unsafe { CStr::from_ptr(from.as_ptr().cast()) }
    }
}

impl<'a> CStrMut<'a> {
    /// Casts mutably from [`core::ffi::CStr`].
    pub const fn from_cstr(from: &'a mut core::ffi::CStr) -> Self {
        // SAFETY: `core::ffi::CStr` represents a valid C string, which is 
        // readable and writeable up to a null terminator
        unsafe { CStrMut::from_ptr(from.as_ptr().cast_mut().cast()) }
    }
}
