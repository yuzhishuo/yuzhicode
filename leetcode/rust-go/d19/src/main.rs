fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: impl Fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    do_twice(|tag: i32| tag + 1, 1);
}
