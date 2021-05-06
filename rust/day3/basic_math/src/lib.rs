/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation. 
/// Code blocks start with triple backticks. The code has an implicit `fn main()` inside and `extern crate <cratename>`,  
/// which means you can just start writing code.
///
/// ```
/// let result = basic_math::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// The primary way to document a Rust library is through annotating the source code with triple forward slashes (///), 
// known as documentation comments. Documentation comments are written in Markdown and support code blocks in them,
// so these code blocks are compiled and used as tests.