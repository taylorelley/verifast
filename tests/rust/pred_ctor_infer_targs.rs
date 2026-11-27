// The type arguments of a generic predicate constructor are inferred from its
// arguments in open and close statements.
/*@

pred_ctor holds<T>(p: *T)() = *p |-> ?v;

pred_ctor holds_any<T>(n: i32)() = true;

lem roundtrip(p: *u16)
    req *p |-> ?v;
    ens *p |-> _;
{
    close holds(p)();
    open holds(p)();
}

lem not_inferable()
    req true;
    ens true;
{
    close holds_any(0)(); //~should_fail
    leak holds_any::<u8>(0)();
}

@*/

fn main() {}
