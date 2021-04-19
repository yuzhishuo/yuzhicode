fn main (){

    let number: u32 = "42".parse().expect("Not is Number");
    //!  ^^^^^^ consider giving `number1` a type
    let number1 = "32".parse().expect("Not is Number");
}