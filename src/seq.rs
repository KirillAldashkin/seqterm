use core::{fmt, marker::PhantomData, ptr::NonNull, slice};

use crate::Term;

/// Reference to a sequence terminated with a special value.
///
/// # Memory layout
/// [`Seq<'a, V, T>`] has the same in-memory representation as `&'a V`.
#[repr(transparent)]
// INVARIANT: `self.0` is a valid pointer to a terminated sequence. We can't
// just use a reference here because we need to store provenance to access all
// subsequent elements.
pub struct Seq<'a, V, T: Term<V>>(
    pub(crate) NonNull<V>,
    pub(crate) PhantomData<(&'a V, T)>,
);

// SAFETY: "`&V is Send` if `V is Sync`"
unsafe impl<'a, V: Sync, T: Term<V>> Send for Seq<'a, V, T> {}

// SAFETY: "`&V is Sync` if `V is Sync`"
unsafe impl<'a, V: Sync, T: Term<V>> Sync for Seq<'a, V, T> {}

impl<'a, V, T: Term<V>> From<Seq<'a, V, T>> for *const V {
    fn from(value: Seq<'a, V, T>) -> Self {
        value.0.as_ptr()
    }
}

impl<'a, V, T: Term<V>> Iterator for Seq<'a, V, T> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        // SAFETY: `self.0` is readable as per type invariant and `Term` requirements
        let current = unsafe { self.0.as_ref() };
        // SAFETY: non-last element in a terminated sequence must have a next one
        *self = Self(unsafe { self.0.add(1) }, PhantomData);
        Some(current)
    }
}

impl<'a, V: fmt::Debug, T: Term<V>> fmt::Debug for Seq<'a, V, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl<'a, V, T: Term<V>> Seq<'a, V, T> {
    /// Creates a new terminated sequence starting at a given pointer.
    /// # Safety
    /// `ptr` must be sequentially readable up to a terminator value 
    /// (which is defined by `T`).
    pub const unsafe fn from_ptr(ptr: *const V) -> Self {
        // SAFETY: pointer must be readable for at least the immediate 
        // terminator as required by this function, therefore it cannot be null
        Self(unsafe { NonNull::new_unchecked(ptr.cast_mut()) }, PhantomData)
    }

    /// Returns the amount of elements in this sequence.
    pub fn len(&self) -> usize {
        let mut len = 0;
        // SAFETY: as per type invariant, must be valid up to a terminator
        while unsafe { !T::is_end(self.0.add(len)) } {
            len += 1;
        }
        len
    }

    /// Returns a slice from current value to a terminator.
    pub fn collect(self) -> &'a [V] {
        // SAFETY: as per type invariant, must be valid up to a terminator
        unsafe { slice::from_raw_parts(self.0.as_ptr(), self.len()) }
    }
}

impl<'a, V, T: Term<V>> Clone for Seq<'a, V, T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<'a, V, T: Term<V>> Copy for Seq<'a, V, T> {}
