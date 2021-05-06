
fn main () {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    
    // In the preceding unit, we mentioned that trying to access a vector's non-existent index would cause the program to panic,
    // but you could avoid that by using the Vec::get method, which returns an Option type instead of panicking. 
    
    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);
    
    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);
    
    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // Pattern matching
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Option::Some(val)=>println!("It's a delicious {}!", val),
            Option::None=>println!("There is no fruit! :("),
        }
    }

    let some_value : Option<u8> = Some(8);
    
    // We want to do something with the Some(7) match but ignore other Some<u8> values or the None variant. 
    // You can add the _ (underscore) wildcard pattern after all other patterns to match anything else, 
    // and it's used to satisfy the compiler demands for exhausting match arms.
    match some_value {
        Some(8) => println!("yes"),
        _=> (),
    }

    if let Some(8) = some_value {
        println!("That's my lucky number!");
    }

    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
    // The expect method does the same as unwrap,
    // but it provides a custom panic message that's provided by its second argument:

    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    let b: Option<&str> = None;
    b.expect("fruits are healthy"); // panics with `fruits are healthy`
}
