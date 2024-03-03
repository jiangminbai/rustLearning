/*
 * crate: 是Rust编译时的最小单位。crate有两种形式：二进制项和库。
 *   1. 二进制项：可被编译成可执行代码。内部必须有一个main函数表示程序执行入口。
 *   2. 库：不会被编译成可执行代码，而是提供一些函数供其他项目也能使用。内部没有main函数。
 * 包：提供一系列功能的一个或多个crate。包中至多包含一个库，和任意个二进制项。
 *   1.如果一个包含 src/main.ts 和 src/lib.rs ，则这两个crate的名字与包名相同。
 * 模块：用来管理代码的组织。包含哪些内容作为私有部分，哪些内容作为公共部分，以及程序每个作用域的名字。
 *   1.声明模块和声明子模块：mod name;
 *   2.声明公有模块: pub mod name; 模块内的代码默认对父模块是私有。
 *   3.引用模块的路径：use crate::pName::cName;
 *     a.绝对路径：以crate根(root)开头的全路径。
 *       i.对于当前crate，以crate字面量开头。
 *       ii.对于外部crate，以crate名开头。引用当前包中的lib.rs的crate名与包名相同。
 *     b.相对路径：从当前模块开始。self|super|定义在当前模块的标识符开始。
 *   4.use
 *     a. as重命名：use crate::io::result as res;
 *     b. pub use重导出名称：pub use crate::front_of_house::hosting;
 *     c. 使用外部包：cargo.toml中的rand: 0.8.6 从crates.io中下载rand及依赖项。
 *     d. 嵌套路径消除大量use行: use std::{cmp:Ordering, io}; use std::io::{self, Write};
 *     e. glob将所有公有定义引入作用域：use std::collections::*; 常用于测试或prelude模式。
 *   5.将模块拆分成多个文件：模块树
 */

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}