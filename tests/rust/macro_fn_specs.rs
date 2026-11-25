// Annotations written in a macro body apply to every expansion of the macro.
macro_rules! constant_fn {
    ($name:ident, $value:expr) => {
        pub fn $name() -> i32
        //@ req true;
        //@ ens result == 1; //~should_fail
        {
            $value
        }
    };
}

constant_fn!(one, 1);
// Without a shared contract, the second expansion would be skipped as specless.
constant_fn!(two, 2);

fn main() {}
