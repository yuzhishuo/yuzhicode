fn main() {
    for cur in (0..=100).filter(|x| x % 5 == 0) {
        println!("{}", cur);
    }
}
