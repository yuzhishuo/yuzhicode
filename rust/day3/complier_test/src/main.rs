fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {

    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10,12), 22);
    assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic]
fn add_works1() {

    assert_eq!(add(1,3), 5);
}

#[test]
#[ignore = "no test"]
fn add_works2() {

}


#[cfg(test)]
mod add_function_tests {

    use super::*;

    #[test]
    fn add_works3() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 21), 32);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4);
    }
}

fn main() {
    println!("Hello, world!");
}
