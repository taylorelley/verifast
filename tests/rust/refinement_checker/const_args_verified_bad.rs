#![feature(core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::{self, AtomicOrdering as AO};

pub unsafe fn load<T: Copy, const VOLATILE: bool>(p: *const T) -> T
//@ req true;
//@ ens true;
{
    unsafe { intrinsics::atomic_load::<T, { AO::SeqCst }, VOLATILE>(p) }
}

fn main() {}
