// Arithmetic operands must have the slot's integer type.
#![feature(core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::{self, AtomicOrdering as AO};

unsafe fn add_wider(p: *mut u16, d: u32) -> u16
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, (std::intrinsics::atomic_value_allowed)(true));
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, (std::intrinsics::atomic_value_allowed)(true));
{
    unsafe { intrinsics::atomic_xadd::<u16, u32, { AO::Relaxed }>(p, d) }
}

fn main() {}
