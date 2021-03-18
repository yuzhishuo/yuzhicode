fn process(input: String) { }

fn caller() {
    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s);
}


fn main () {
    caller();
}