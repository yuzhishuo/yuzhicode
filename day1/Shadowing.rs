fn main () {

    // You can also declare a new variable with the same name as a previous variable,
    // which creates a new binding.
    // In Rust, this operation is called "shadowing" because the new variable shadows the previous variable.
    let number = 5; 
    let number = number + 5;
    let number = number * 2;
    println!("the number is: {}", number);
}

