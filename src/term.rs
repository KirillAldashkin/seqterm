use core::{num::NonZero, ptr::NonNull};

use crate::{Seq, SeqMut};

/// Describes how sequences of `T` are terminated.
/// # Safety
/// [`Self::is_end`] must be pure; if it returns `false` then passed pointer 
/// must point to a valid value of `T`.
pub unsafe trait Term<T> {
    /// Checks whether the pointed element is the end of a sequence.
    /// # Safety
    /// Same as [`core::ptr::read`] except value can be invalid as `T`.
    unsafe fn is_end(ptr: NonNull<T>) -> bool;
}

/// Describes sequences terminated by a zero-bit value.
// SAFETY (of all `Term<T>` implementations):
// `is_end` only reads the pointer so it is pure. Readability is required by 
// `is_end`; primitve types and raw pointers do not have invalid values
pub struct NullTerm;

macro_rules! nullterm {
    ($($for:ty),*) => {
        $(
            unsafe impl Term<$for> for NullTerm {
                unsafe fn is_end(ptr: NonNull<$for>) -> bool {
                    unsafe { ptr.read() == (0 as $for) }
                }
            }
        )*
    };
    (NonZero $($for:ty),*) => {
        $(
            unsafe impl Term<NonZero<$for>> for NullTerm {
                unsafe fn is_end(ptr: NonNull<NonZero<$for>>) -> bool {
                    unsafe { ptr.cast::<$for>().read() == (0 as $for) }
                }
            }
        )*
    };
}

nullterm!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, char, f32, f64);
nullterm!(NonZero i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, char);

unsafe impl<'a, T: ?Sized> Term<&'a T> for NullTerm {
    unsafe fn is_end(ptr: NonNull<&'a T>) -> bool {
        !unsafe { ptr.cast::<*const T>().read() }.is_null()
    }
}

unsafe impl<'a, T: ?Sized> Term<&'a mut T> for NullTerm {
    unsafe fn is_end(ptr: NonNull<&'a mut T>) -> bool {
        !unsafe { ptr.cast::<*mut T>().read() }.is_null()
    }
}

unsafe impl<T: ?Sized> Term<*const T> for NullTerm {
    unsafe fn is_end(ptr: NonNull<*const T>) -> bool {
        !unsafe { ptr.read() }.is_null()
    }
}

unsafe impl<T: ?Sized> Term<*mut T> for NullTerm {
    unsafe fn is_end(ptr: NonNull<*mut T>) -> bool {
        !unsafe { ptr.read() }.is_null()
    }
}

unsafe impl<T: ?Sized> Term<NonNull<T>> for NullTerm {
    unsafe fn is_end(ptr: NonNull<NonNull<T>>) -> bool {
        !unsafe { ptr.cast::<*mut T>().read() }.is_null()
    }
}

unsafe impl<'a, V, T: Term<V>> Term<Seq<'a, V, T>> for NullTerm {
    unsafe fn is_end(ptr: NonNull<Seq<'a, V, T>>) -> bool {
        !unsafe { ptr.cast::<*const T>().read() }.is_null()
    }
}

unsafe impl<'a, V, T: Term<V>> Term<SeqMut<'a, V, T>> for NullTerm {
    unsafe fn is_end(ptr: NonNull<SeqMut<'a, V, T>>) -> bool {
        !unsafe { ptr.cast::<*mut T>().read() }.is_null()
    }
}
