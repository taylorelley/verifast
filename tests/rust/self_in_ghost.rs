// Ghost code in an inherent impl can refer to the impl's self type as Self.
pub struct Wrap<T> {
    x: T,
}

impl Wrap<u16> {
    pub fn size() -> usize
    //@ req true;
    //@ ens result == std::mem::size_of::<Self>();
    {
        std::mem::size_of::<Wrap<u16>>()
    }
}

impl<T> Wrap<T> {
    pub fn identity(p: *mut Wrap<T>) -> *mut Wrap<T>
    //@ req true;
    //@ ens result == p as *Self;
    {
        p
    }
}

// In a macro, Self is the only way for ghost code to name each expansion's type.
macro_rules! sized {
    ($t:ty) => {
        impl Wrap<$t> {
            pub fn size_of_self() -> usize
            //@ req true;
            //@ ens result == std::mem::size_of::<Self>();
            {
                std::mem::size_of::<Wrap<$t>>()
            }
        }
    };
}

sized!(u8);
sized!(u64);

impl Wrap<u32> {
    pub fn wrong_size() -> usize
    //@ req true;
    //@ ens result == std::mem::size_of::<Self>(); //~should_fail
    {
        std::mem::size_of::<Wrap<u8>>()
    }
}

// Self inside a string literal is not the self type.
impl Wrap<u16> {
    pub fn literal()
    //@ req true;
    //@ ens length("Self") == 4;
    {
    }
}

// An impl whose self type ghost code cannot name still verifies when the
// annotations do not use Self.
pub enum Borrowed<'a> {
    Byte(&'a u8),
}

impl<'a> Borrowed<'a> {
    pub fn unused_self()
    //@ req true;
    //@ ens true;
    {
    }
}

fn main() {}
