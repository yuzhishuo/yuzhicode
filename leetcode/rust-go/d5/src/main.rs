
struct A;
struct S(A);
struct SGent<T>(T);
fn reg_fn (_s: S) {}
#[allow(dead_code)]
fn gen_fn (_sg: SGent<A>) {}

fn generic<T> (_s: SGent<T>) {}
fn main() {

    reg_fn(S(A));
    generic(SGent(A));
    generic(SGent(1));

    generic::<f32>(SGent(1.0));
}
