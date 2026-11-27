// A struct whose field type is a projection of an associated type with an
// explicit or implicit Sized bound.
use std::cell::UnsafeCell;

pub trait Primitive {
    type Storage;
}

pub trait ExplicitlySized {
    type Storage: Sized;
}

impl Primitive for u8 {
    type Storage = u8;
}

#[repr(C)]
pub struct Cell<T: Primitive> {
    v: UnsafeCell<T::Storage>,
}

#[repr(C)]
pub struct Cell2<T: ExplicitlySized> {
    v: T::Storage,
}

// A projection of an associated type of a trait declared in an .rsspec file.
pub struct Peeked<I: Iterator> {
    item: Option<I::Item>,
    iter: I,
}

pub fn smoke()
//@ req true;
//@ ens true;
{}
