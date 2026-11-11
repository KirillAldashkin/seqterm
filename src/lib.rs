#![no_std]
#![doc = include_str!("../README.md")]

mod term;
mod seq;
mod seqmut;
mod cstr;

pub use term::{Term, NullTerm};
pub use seq::Seq;
pub use seqmut::SeqMut;
pub use cstr::{CChar, CStr, CStrMut};
