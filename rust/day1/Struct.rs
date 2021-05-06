// A struct with named fields
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

// A tuple struct
struct Point2D(u32, u32);

// A unit struct
struct Unit;


// Classic C structs are the most commonly used. Each field defined within them has a name and a type. After they're defined, they can be accessed by using example_struct.field syntax.
// Tuple structs are similar to classic structs, but their fields have no names. For accessing individual variables, the same syntax is used as with regular tuples, namely, foo.0, foo.1, and so on, starting at zero.
// Unit structs are most commonly used as markers. They're useful when you need to implement a trait on something but don't need to store any data inside it.


struct Ex {
    tuple :(String, i32, u32),
}
fn main () {

    let person  = Person{ 
        name: String::from("Yimin Liu"),
        likes_oranges: true,
        age: 23,
    };

    let origin = Point2D(0, 0);

    // Instantiate a unit struct.
    let unit = Unit;
    
    // Ex
    let e = Ex{ tuple: ("Yimin Liu".to_string(), 18, 23) };
}