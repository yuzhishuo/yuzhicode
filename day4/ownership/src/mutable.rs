fn main () {

    let mut value = String::from("hello");

    
    let ref1 = &mut value;
    let ref2 = &mut value;
    
    // cannot borrow `value` as mutable more than once at a time
    print!("{}, {}", ref1, ref2);
}