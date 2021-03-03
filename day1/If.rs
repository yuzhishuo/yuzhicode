fn main () {

    if 1 == 2 {
        println!("whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }

    let formal = true;
    let greeting = if formal {
            "Good evening."
        } else {
            "Hello, friend!"
        };
    println!("{}", greeting); // prints "Good evening."

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // loop {
    //     println!("I loop forever");
    // }


    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
        break i;
        }
    };
    assert_eq!(something, 128);

    let mut counter = 0;

    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for item in 0..5 {
        println!("{}", item * 2);
    }
}