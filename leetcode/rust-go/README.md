# 参考书籍

[rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/title-page.html)

[The Rustonomicon](https://learnku.com/docs/nomicon/2018)

[Async programming in Rust with async-std](https://learnku.com/docs/rust-async-std)

[The Cargo Book](https://learnku.com/docs/cargo-book/2018)

## R9
多线程使用通道进行通信

## R10
发送多个值并观察接收者的等待

## R11
在单线程使用锁

## R12
在多线程中使用锁 (原子引用计数 std::async::Arc)

### `RefCell<T>/Rc<T>` 与 `Mutex<T>/Arc<T>` 的相似性
你可能注意到了，因为 counter 是不可变的，不过可以获取其内部值的可变引用；这意味着 Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样。正如第十五章中使用 RefCell<T> 可以改变 Rc<T> 中的内容那样，同样的可以使用 Mutex<T> 来改变 Arc<T> 中的内容。

另一个值得注意的细节是 Rust 不能避免使用 Mutex<T> 的全部逻辑错误。回忆一下第十五章使用 Rc<T> 就有造成引用循环的风险，这时两个 Rc<T> 值相互引用，造成内存泄漏。同理，Mutex<T> 也有造成 死锁（deadlock） 的风险。这发生于当一个操作需要锁住两个资源而两个线程各持一个锁，这会造成它们永远相互等待。如果你对这个主题感兴趣，尝试编写一个带有死锁的 Rust 程序，接着研究任何其他语言中使用互斥器的死锁规避策略并尝试在 Rust 中实现他们。标准库中 Mutex<T> 和 MutexGuard 的 API 文档会提供有用的信息。


