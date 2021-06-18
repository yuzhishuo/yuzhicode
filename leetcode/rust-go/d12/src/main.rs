fn main() {
    let immutable_box = Box::new(123);

    println!("this box is {}", immutable_box);

    let mut mutable_box = immutable_box;

    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}
