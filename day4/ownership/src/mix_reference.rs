fn main() {
    let mut value = String::from("hello");

    let ref1 = &value;
                //------ immutable borrow occurs here
    let ref2 = &mut value;
                //  ^^^^^^^^^^ mutable borrow occurs here
    println!("{}, {}", ref1, ref2);
                //  ---- immutable borrow later used here
}