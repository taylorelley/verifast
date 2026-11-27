// An illegal ordering at an atomic intrinsic call is a static error.
#![feature(core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::{self, AtomicOrdering as AO};

unsafe fn release_load(p: *mut u8) -> u8
//@ req [?f]std::intrinsics::atomic_points_to(p, ?perm, ?inv_);
//@ ens [f]std::intrinsics::atomic_points_to(p, perm, inv_);
{
    unsafe { intrinsics::atomic_load::<u8, { AO::Release }, false>(p) }
}

fn main() {}
