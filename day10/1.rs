use std::convert::{From, Into};

struct Muduo;

impl Into<String> for Muduo {
    fn into(self) -> String {
        String::from("muduo")
    }
}

fn main (){

    let mf = Muduo{};
    let t: String = mf.into();
    println!("{}", t);
}