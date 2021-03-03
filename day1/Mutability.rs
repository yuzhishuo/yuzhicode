fn main() {

    let a_number = 10;
    println!("this numbser is {}", a_number);

    a_number = 15; //! cannot assign twice to immutable variable `a_number`
    // When a variable is immutable, after a value is bound to a name, you can't change that value.
    println!("and now the number is {}", a_number);
    
}