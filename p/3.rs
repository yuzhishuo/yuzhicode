use std::env;

fn double_arg(mut argv: env::Args, ) ->Result<i32, String> {

    argv.nth(1)
        .ok_or("give one".to_owned())
        .and_then(|arg| arg.parse::<i32>()
        .map_err(|error| error.to_string()))
        .map(|n| n*2)
}


fn main () {

    match  double_arg(std::env::args()) {
        Ok(n) => println!("{}", n),
        Err(error) => println!("{}", error)
    }
}