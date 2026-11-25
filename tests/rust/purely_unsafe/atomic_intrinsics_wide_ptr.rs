// A wide pointer operand is rejected, also when its pointee is generic.
#![feature(core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::{self, AtomicOrdering as AO};

unsafe fn swap_wide<T: ?Sized>(p: *mut *mut T, v: *mut T) -> *mut T
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, ?inv_) &*& inv_(v) == true;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, inv_);
{
    unsafe { intrinsics::atomic_xchg::<*mut T, { AO::AcqRel }>(p, v) }
}

fn main() {}
