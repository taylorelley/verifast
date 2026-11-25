// Atomic intrinsics with const-generic orderings, for concrete and generic operands.
#![feature(core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::{self, AtomicOrdering as AO};

unsafe fn load_u16(p: *mut u16) -> u16
//@ req [?f]std::intrinsics::atomic_points_to(p, ?perm, ?inv_);
//@ ens [f]std::intrinsics::atomic_points_to(p, perm, inv_) &*& inv_(result) == true;
{
    unsafe { intrinsics::atomic_load::<u16, { AO::Acquire }, false>(p) }
}

unsafe fn store<T: Copy, const VOLATILE: bool>(p: *mut T, v: T)
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, ?inv_) &*& inv_(v) == true;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, inv_);
{
    unsafe { intrinsics::atomic_store::<T, { AO::Release }, VOLATILE>(p, v) }
}

unsafe fn swap<T: Copy>(p: *mut T, v: T) -> T
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, ?inv_) &*& inv_(v) == true;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, inv_) &*& inv_(result) == true;
{
    unsafe { intrinsics::atomic_xchg::<T, { AO::AcqRel }>(p, v) }
}

// The pointee is generic but Sized, so the pointer operand is thin.
unsafe fn swap_ptr<T>(p: *mut *mut T, v: *mut T) -> *mut T
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, ?inv_) &*& inv_(v) == true;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, inv_) &*& inv_(result) == true;
{
    unsafe { intrinsics::atomic_xchg::<*mut T, { AO::AcqRel }>(p, v) }
}

unsafe fn store_read_only(p: *mut u16)
//@ req [?f]std::intrinsics::atomic_points_to(p, ?perm, ?inv_) &*& perm != 1 &*& inv_(0) == true;
//@ ens true;
{
    unsafe { intrinsics::atomic_store::<u16, { AO::Relaxed }, false>(p, 0) } //~should_fail
}

fn main() {}
