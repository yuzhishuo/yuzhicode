fn longest_word<'a>(x: str, y: str) -> &'a str {
    if x.len() > y.len() {
        String::from(x + y)
    } else {
        String::from(y + x)
    }
}

fn main() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(magic1, magic2);
    println!("The longest magic word is {}", result);
}
