use std::mem::size_of;
// use std::any::type_name_of_val;
use std::mem::size_of_val;

fn main () {

  let number : isize = 1;
  let number : usize = 2;

  println!("{}", size_of::<isize>());
  println!("{}", size_of::<usize>());

  println!("{}", 3.0f64);
  println!("{}", 3.0f32);

  let numbe1  = (1 - 10);

  println!("{}", size_of_val(&numbe1));

  // println!("{}", type_name_of_val(&11));
}