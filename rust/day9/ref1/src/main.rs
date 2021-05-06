fn main() {
    let reference = &3i32;

    match *reference {
        val => println!("{}", val),
    }

    match &reference {
        &val => println!("{}", val),
    }

    let value = 3i32;

    match value {
        ref val => println!("{}", val),
    }
}
