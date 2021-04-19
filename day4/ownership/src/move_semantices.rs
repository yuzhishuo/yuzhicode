fn process(input: String) { }

fn caller() {
    let s = String::from("Hello, world!");
    // - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    process(s; // Transfers ownership of `s` to `process`
    process(s); // Error! ownership already transferred.
}


fn main () {
    caller();
}