use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {

    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        write!(f, "(MinMax {},{})", self.0, self.1)
    }
}

fn main () 
{
    // 1 of 10 people know binary, the other half doesn't
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);


    //     1, abc 
    println!("{number:>width$}, abc", number=1, width=6);

    #[derive(Debug)]
    struct DebugPrintAble(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintAble);


    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    println!("Now {:?} is printing", DebugPrintAble(3));

    println!("Now {:?} is printing", Deep(DebugPrintAble(3)));

    #[derive(Debug)]
    struct Person <'a> {

        name : &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("Persion is {person:#?}", person = &peter);

    let min_max = MinMax(1, 2);

    print!("{}", min_max);
    
    print!("{:?}", min_max);


}