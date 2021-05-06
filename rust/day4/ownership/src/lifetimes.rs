fn main() {
    let x;
    {
        let y = 42;
        x = &y;
        // ^^ borrowed value does not live long enough
    }
    // - `y` dropped here while still borrowed
    println!("x: {}", x);
                    //  - borrow later used here
}