
fn main () {


    // array

    let weekdays = ["Monday", "Thusday", "WednesDay", "Thursday", "Friday", "Saturday", "Sunday"];

    let byte_buffer = [0_u8; 512];

    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("first element of the array: {}", letters[0]);  // prints 'a'
    println!("second element of the array: {}", letters[1]); // prints 'b'

    
    // You might have noticed the {:?} format parameter inside the println! calls. 
    // It's used whenever we want to print something for debugging reasons, whereas {} is used for displaying information to a user. 
    // Because we didn't tell Rust how to represent a vector of integers to users, using the former mark would result in a compilation error.
    // We're going to learn precisely how to do that when we reach the "Traits" module in this course.
    
    let there_element = vec![1,2,3,];
    println!("Initial vector: {:?}", there_element);  // prints "[1, 2, 3]"


    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(5);

    println!("Initial vector: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', Collect.rs:34:26
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // let does_not_exist = v[100];

    let mut book_reviews = std::collections::HashMap::new();
    book_reviews.insert(3, 5);

    if !book_reviews.contains_key(&3) {
        println!("We've got {} reviews, but Les Misérables ain't one.",
        book_reviews.len());
    }
    
    book_reviews.remove(&3);

    indexing_tuple();
    indexing_array();

    let basket = fruit_basket();

    assert!(
    basket.len() >= 3,
    "basket must have at least three types of fruits"
    );
    assert!(
    basket.values().sum::<u32>() >= 5,
    "basket must have at least five fruits"
    );
}

fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(
    2, second,
    "This is not the 2nd number in the tuple: {}",
    second
    )
}

fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    // Replace below ??? with the array indexing syntax.
    let letter_d = characters[3];

    assert_eq!(
    'd', letter_d,
    "This is not the character for the letter d: {}",
    letter_d
    )
}

fn fruit_basket() -> std::collections::HashMap<String, u32> {
    let mut basket = std::collections::HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2_u32);

    basket.insert(String::from("apple"), 3_u32);

    basket.insert(String::from("peach"), 4_u32);

    return basket;
}
