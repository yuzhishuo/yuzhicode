fn life_circul<'a, 'b>() {
    let b = 1;

    // let ref c :&'a i32 = &b;
}

fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

// 声明周期要优先于 泛型
fn pass_y<'a, 'b, T, U>(x: &'a T, y: &'b U) {}

fn main() {}
