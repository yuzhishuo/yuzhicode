fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Hello from another function!");
    assert_eq!(is_divisible_by(2, 3), false);
    assert_eq!(is_divisible_by(5, 1), true);
    assert_eq!(is_divisible_by(24, 6), true);
}

fn is_divisible_by (dividend: u32, divisor: u32) -> bool {

    if divisor == 0 {
        return false;
    }

    dividend % divisor == 0
}