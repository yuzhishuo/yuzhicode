
fn main() {

    let mut num = 1;
    let p1 = &num as *const i32;
    let p2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *p1);
        *p2 = 2;
        println!("p2 is: {}", *p2);
    }
}
