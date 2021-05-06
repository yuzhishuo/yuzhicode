fn main() {
    let mut name = vec!["tianyi", "yimin", "huangqiang"];

    for item in name.iter_mut() {
        *item = match item {
            &mut "huangqiang" => "heihei",
            &mut val => "val",
            _ => "heihei",
        }
    }

    println!("{:?}", name);
}
