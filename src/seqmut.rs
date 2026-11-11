use core::{fmt, marker::PhantomData, ptr::NonNull, slice};

use crate::{Seq, Term};

/// Mutable reference to a sequence terminated with a special value.
///
/// # Memory layout
/// [`SeqMut<'a, V, T>`] has the same in-memory representation as `&'a mut V`.
#[repr(transparent)]
// INVARIANT: `self.0` is a valid pointer to a terminated sequence. We can't
// just use a reference here because we need to store provenance to read all
// subsequent elements.
pub struct SeqMut<'a, V, T: Term<V>>(
    pub(crate) NonNull<V>,
    pub(crate) PhantomData<(&'a mut V, T)>,
);

// SAFETY: "`&mut V is Send` if `V is Send`"
unsafe impl<'a, V: Send, T: Term<V>> Send for SeqMut<'a, V, T> {}

// SAFETY: "`&mut V is Sync` if `V is Sync`"
unsafe impl<'a, V: Sync, T: Term<V>> Sync for SeqMut<'a, V, T> {}

impl<'a, V, T: Term<V>> From<SeqMut<'a, V, T>> for *mut V {
    fn from(value: SeqMut<'a, V, T>) -> Self {
        value.0.as_ptr()
    }
}

impl<'a, V, T: Term<V>> Iterator for SeqMut<'a, V, T> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        // SAFETY: `self.0` is readable as per type invariant and `Term` requirements
        let current = unsafe { self.0.as_mut() };
        // SAFETY: non-last element in a terminated sequence must have a next one
        *self = Self(unsafe { self.0.add(1) }, PhantomData);
        Some(current)
    }
}

impl<'a, V: fmt::Debug, T: Term<V>> fmt::Debug for SeqMut<'a, V, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.borrow()).finish()
    }
}

impl<'a, V, T: Term<V>> SeqMut<'a, V, T> {
    /// Creates a new terminated sequence starting at a given pointer.
    /// # Safety
    /// `ptr` must be sequentially readable and writeable up to a terminator
    /// value (which is defined by `T`).
    pub const unsafe fn from_ptr(ptr: *mut V) -> Self {
        // SAFETY: pointer must be readable for at least the immediate 
        // terminator as required by this function, therefore it cannot be null
        Self(unsafe { NonNull::new_unchecked(ptr) }, PhantomData)
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
    pub fn collect(self) -> &'a mut [V] {
        // SAFETY: as per type invariant, must be valid up to a terminator
        unsafe { slice::from_raw_parts_mut(self.0.as_ptr(), self.len()) }
    }

    /// Clones this reference via a mutable borrow.
    pub const fn borrow_mut<'b>(&'b mut self) -> SeqMut<'b, V, T> {
        Self(self.0, PhantomData)
    }

    /// Clones this reference via an immutable borrow.
    pub const fn borrow<'b>(&'b self) -> Seq<'b, V, T> {
        Seq(self.0, PhantomData)
    }
}
