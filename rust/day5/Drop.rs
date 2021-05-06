struct PrintOrDrop (&'static str);

impl Drop for PrintOrDrop {

    fn drop (&mut self ) {
        println!("{}", self.0);
    }
}

// 先销毁第二个参数, 接下来是 `y`, 然后是第一个参数r, 最后是 `x`
fn patterns_in_parameters(
    (x, _): (PrintOrDrop, PrintOrDrop),
    (_, y): (PrintOrDrop, PrintOrDrop),
) {}

fn main () 
{
    let mut override_writen_fail = PrintOrDrop("123");
    override_writen_fail = PrintOrDrop("456");
    // 在部分移动之后，后续销毁动作只销毁剩余字段。
    let mut partial_move = (PrintOrDrop("first"), PrintOrDrop("forgotten"));
    // 执行部分移出，只留下 `partial_move.0` 处于初始化状态
    core::mem::forget(partial_move.1);
    // 当 partial_move 的作用域结束时, 这里就只有第一个字段被销毁。


    // 销毁顺序是 3 2 0 1
    patterns_in_parameters(
        (PrintOrDrop("0"), PrintOrDrop("1")),
        (PrintOrDrop("2"), PrintOrDrop("3")),
    );

    let declared_first = PrintOrDrop("在外层作用域内最后销毁");
    {
        let declared_in_block = PrintOrDrop("在内层作用域内销毁");
    }
    let declared_last = PrintOrDrop("在外层作用域内最先销毁");


    // 在此条语句的末尾处销毁
    (PrintOrDrop("first operand").0 == ""
    // 在 ) 处销毁
    || PrintOrDrop("second operand").0 == "")
    // 在此表达式的末尾处销毁
    || PrintOrDrop("third operand").0 == "";

    const _: () =  { struct _SameNameTwice; };

    // OK 尽管名称和上面的一样：
    const cc: () =  { struct _SameNameTwice; };

    println!("{:?}", cc );
}