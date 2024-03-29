/*
 * 1.栈和堆存储数据的区别
 *   a.栈：数据大小是固定的。
 *   b.堆：数据大小是未知的。在堆上分配内存会返回一个指针，指针大小是已知的，被存在栈中。
 * 2.两种字符串类型：字符串字面量和String
 *   a.字符串字面量：数据大小是固定的。被存在栈中。
 *   b.String类型：数据大小是未知的。被存在堆中。
 * 3.所有权规则
 *   a.Rust中的每一个值都有一个所有者；
 *   b.值在任意时刻有且仅有一个所有者；
 *   c.当所有者(变量)离开作用域，这个值将被丢弃。
 * 4.所有权特点
 *   a.被存储在堆上的数据不能被多次引用。
 * 5.引用与指针
 *   引用像一个指针，是一个地址，指向某个类型的有效值。使用值但不获取所有权。
 * 6.引用类型
 *   a.不可变引用
 *   b.可变引用
 *   c.垂悬引用
 * 7.引用的规则
 *   a.任意时间，要么只能有一个可变引用，要么只能有多个不可变引用
 * 8.slice
 */
fn main() {
    let s = String::from("hello world");
    let first = &s[0..5];
    let second = &s[6..11];
    println!("{}, {}", first, second);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}