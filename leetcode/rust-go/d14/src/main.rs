struct Fs(i32);
#[allow(unused_variables)]
fn main() {
    let t = Fs(1);
    let ref t1 = t;

    let t2 = t1; // 可以看出引用具有拷贝语义
    println!("{}", t1.0);
    println!("{}", t2.0); // 引用的赋值 是拷贝语义

    let mut f = Fs(2);
    let ref mut f1 = f; // 可变引用的赋值 是移动语义
    let f2 = f1;
    f2.0 = 3;
    // println!("{}", f1.0);
}
