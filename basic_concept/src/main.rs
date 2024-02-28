/**
 * 1. 变量的可变性
 *    a. 可变变量不能改变变量类型
 *    b.变量的隐藏可以改变变量类型
 * 2. 数据类型有两种：标量和复合
 * 标量：整型、浮点型、字符、布尔值
 * 复合：
 *    a. 元组：可以包含不同类型的数据
 *    b. 数组：只能包含同种类型的数据，数组长度固定
 * 3. 函数
 * 4. 语句和表达式
 *    a. 语句：执行一些操作但不返回值。比如：函数定义
 *    b. 表达式： 计算并产生一个值。它可以被赋值给变量。比如：数值运算、箭头函数调用、大括号块级作用域等
 * 5. 控制语句
 *    a. if
 *    b. loop
 *    c. while
 *    d. for
 */
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for ele in a {
        println!("the value is: {}", ele);
    }
}
