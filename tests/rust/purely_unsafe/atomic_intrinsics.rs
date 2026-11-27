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

// Both compare-exchange forms preserve the slot invariant and return a valid value.
unsafe fn cas_u32(p: *mut u32, old: u32, new: u32) -> u32
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, ?inv_) &*& inv_(old) == true &*& inv_(new) == true;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, inv_) &*& inv_(result) == true;
{
    let (strong, _) = unsafe { intrinsics::atomic_cxchg::<u32, { AO::AcqRel }, { AO::Acquire }>(p, old, new) };
    let (weak, _) = unsafe { intrinsics::atomic_cxchgweak::<u32, { AO::Release }, { AO::Relaxed }>(p, old, new) };
    if strong == weak { strong } else { weak }
}

// Arithmetic on an integer slot, and on a pointer slot with a usize delta.
unsafe fn add_i64(p: *mut i64, d: i64) -> i64
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, (std::intrinsics::atomic_value_allowed)(true));
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, (std::intrinsics::atomic_value_allowed)(true));
{
    unsafe { intrinsics::atomic_xsub::<i64, i64, { AO::SeqCst }>(p, d) };
    unsafe { intrinsics::atomic_xadd::<i64, i64, { AO::Relaxed }>(p, d) }
}

unsafe fn add_ptr(p: *mut *mut u8, d: usize) -> *mut u8
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, (std::intrinsics::atomic_value_allowed)(true));
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, (std::intrinsics::atomic_value_allowed)(true));
{
    unsafe { intrinsics::atomic_xadd::<*mut u8, usize, { AO::AcqRel }>(p, d) }
}

// Signed and unsigned extrema keep a slot invariant that admits the operand.
unsafe fn extrema(p: *mut i16, q: *mut u16, a: i16, b: u16) -> u16
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, ?inv1) &*& inv1(a) == true &*& [?g]std::intrinsics::atomic_points_to(q, 1, ?inv2) &*& inv2(b) == true;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, inv1) &*& [g]std::intrinsics::atomic_points_to(q, 1, inv2) &*& inv2(result) == true;
{
    unsafe { intrinsics::atomic_max::<i16, { AO::Acquire }>(p, a) };
    unsafe { intrinsics::atomic_min::<i16, { AO::Release }>(p, a) };
    unsafe { intrinsics::atomic_umin::<u16, { AO::Relaxed }>(q, b) };
    unsafe { intrinsics::atomic_umax::<u16, { AO::SeqCst }>(q, b) }
}

// The u8 bitwise contracts also accept a byte restricted to 0 and 1, given a 0 or 1 operand.
unsafe fn bool_byte_ops(p: *mut u8, b: u8) -> u8
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, std::intrinsics::atomic_bool_byte) &*& b == 0 || b == 1;
//@ ens [f]std::intrinsics::atomic_points_to(p, 1, std::intrinsics::atomic_bool_byte) &*& result == 0 || result == 1;
{
    unsafe { intrinsics::atomic_and::<u8, u8, { AO::Relaxed }>(p, b) };
    unsafe { intrinsics::atomic_or::<u8, u8, { AO::Release }>(p, b) };
    unsafe { intrinsics::atomic_xor::<u8, u8, { AO::SeqCst }>(p, b) }
}

// Wider types and NAND use the general contract, which needs an unrestricted slot.
unsafe fn nand_bool_byte(p: *mut u8)
//@ req [?f]std::intrinsics::atomic_points_to(p, 1, std::intrinsics::atomic_bool_byte);
//@ ens true;
{
    unsafe { intrinsics::atomic_nand::<u8, u8, { AO::Relaxed }>(p, 1) }; //~should_fail
}

unsafe fn store_read_only(p: *mut u16)
//@ req [?f]std::intrinsics::atomic_points_to(p, ?perm, ?inv_) &*& perm != 1 &*& inv_(0) == true;
//@ ens true;
{
    unsafe { intrinsics::atomic_store::<u16, { AO::Relaxed }, false>(p, 0) } //~should_fail
}

fn main() {}
