// A generic predicate constructor applied in a generic lemma's contract. Its
// arguments must be checked against its instantiated parameter types.
struct Wrap<T> {
    x: T,
}

/*@

pred_ctor holds<T>(p: *T)(;) = *p |-> ?v;

lem wrap_holds<V>(p: *Wrap<V>)
    req [?f]holds::<Wrap<V> >(p)();
    ens [f](*p |-> ?v);
{
    open [f]holds::<Wrap<V> >(p)();
}

lem wrap_holds_inferred<V>(p: *Wrap<V>)
    req [?f]holds(p)();
    ens [f](*p |-> ?v);
{
    open [f]holds::<Wrap<V> >(p)();
}

lem wrap_holds_bad<V>(p: *Wrap<V>, q: *V)
    req [?f]holds::<Wrap<V> >(q)(); //~should_fail
    ens true;
{
    assume(false);
}

@*/

fn main() {}
