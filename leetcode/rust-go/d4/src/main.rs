use std::fmt; // for fmt::Display

struct List(Vec<i32>);
/*
* 对一个结构体来说，须对各个元素逐个实现 fmt::Display 可能会很麻烦。问题在于每个 write! 都要生成一个 fmt::Result。彻底地实现需要处理所有的结果。
* 出于这方面考虑，Rust 提供了 try! 宏。
* 请使用 r#write 替代 try!
*/

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let List(ref vec) = *self;

        r#write!(f, "[");

        for (counter, v) in vec.iter().enumerate() {
            if counter != 0 {
                r#write!(f, ", ");
            }
            r#write!(f, "{}", v);
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
