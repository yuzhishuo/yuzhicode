fn main() {
    println!("Hello, world!");

    // `mascot` is not valid here, because it's not yet declared.
    {
        let mascot = "ferris";   // `mascot` is valid from this point forward.
        // do stuff with `mascot`.
    }
    // this scope is now over, so `mascot` is no longer valid.
}
