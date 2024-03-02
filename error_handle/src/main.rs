/*
 * rust错误两大类：可恢复的错误和不可恢复的错误
 * 1.不可恢复的错误：立即停止程序 -> panic!
 * 2.可恢复的错误：向用户报告错误并恢复操作 -> Result<T, E>
 *   a.match Rsult<T, E>
 *   b.unwrap和expect
 *     i.Result<T, E>.unwrap().Ok时，返回ok中的值；Err时，调用panic!(默认信息)
 *     ii.Result<T, E>.expect('info').Ok时，调用Ok中的值；Err时，调用panic!(自定义信息)。这更常用
 *   c.传播错误
 *     i.return Err(e)
 *     ii.?传播错误简写:?运算符。与match Result<T, E>作用一样。但是有不一样的地方：转换了Error类型，使其可以使用链式调用
 * 
 */
// use std::{fs::File, io::ErrorKind};
use std::fs::File;
use std::io;
fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };
    
    // println!("{:?}", greeting_file);

    // read_username_from_file()
    //     .expect("Failed to read username");

    let greeting_file = File::open("hello.txt");
    match greeting_file {
        Ok(file) => println!("{:?}", file),
        Err(error) => println!("{:?}", error),
    }
}

// fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // let mut usename_file_result = File::open("hello.text")?;
    // let mut username = String::new();
    // usename_file_result.read_to_string(&mut username)?;
    // Ok(username)

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // fs::read_to_string("hello.txt")
// }