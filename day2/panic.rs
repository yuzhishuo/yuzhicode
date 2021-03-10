fn main () {
    //! This program would exit with status code 101 and print the following message:
    panic!("Farewell!");

    let mut v = vec![0, 1, 2, 3];

    print!("{}", v[6]); // this will cause a panic!
}

