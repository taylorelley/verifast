// Rustc's alignment for instances of a generic struct that appear in local
// function signatures.
#[repr(C, align(2))]
pub struct Align2<T>(T);

#[repr(C)]
pub struct Wrap<T> {
    x: T,
}

pub fn concrete(_p: *mut Align2<u8>, _q: *mut Wrap<u64>)
//@ req true;
//@ ens true;
{
    //@ assert std::mem::align_of::<Align2<u8> >() == 2;
    //@ assert std::mem::align_of::<Wrap<u64> >() == 8;
}

// The layout of Wrap<*mut T> does not depend on T: Sized.
pub fn generic<T>(_p: *mut Wrap<*mut T>)
//@ req true;
//@ ens true;
{
    //@ assert std::mem::align_of::<Wrap<*mut T> >() == std::mem::size_of::<usize>();
}

pub fn not_exported(_p: *mut Wrap<u32>)
//@ req true;
//@ ens true;
{
    //@ assert std::mem::align_of::<Wrap<u16> >() == 2; //~should_fail
}

// A bound on an associated type can fix the layout of an instance in one
// function. The fact must not be generalized to every type argument.
pub trait Primitive {
    type Storage;
}

#[repr(C)]
pub struct Cell<T: Primitive> {
    v: T::Storage,
}

pub fn narrowed<T: Primitive<Storage = u8>>(_p: *mut Cell<T>)
//@ req true;
//@ ens true;
{
    //@ assert std::mem::align_of::<Cell<T> >() == 1; //~should_fail
}

pub fn widened<T: Primitive<Storage = u64>>(_p: *mut Cell<T>)
//@ req true;
//@ ens true;
{
    //@ assert std::mem::align_of::<Cell<T> >() == 8; //~should_fail
}

fn main() {}
