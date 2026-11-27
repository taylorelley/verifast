// The signed extrema require a signed operand type.
#![feature(core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::{self, AtomicOrdering as AO};

unsafe fn signed_max_of_unsigned(p: *mut u8, v: u8) -> u8
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, ?inv_) &*& inv_(v) == true;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, inv_);
{
    unsafe { intrinsics::atomic_max::<u8, { AO::Relaxed }>(p, v) }
}

fn main() {}
