use std::collections::VecDeque;

fn main() {
    for cur in (0..=100).map(|x| (x * x).to_string()) {
        println!("{}", cur);
    }

    let t = (0..=100).collect::<VecDeque<_>>();
}
