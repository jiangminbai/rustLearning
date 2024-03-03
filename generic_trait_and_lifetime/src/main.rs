/*
 * 泛型: 类型参数化
 * Trait: 定义共同的行为，类似于interface
 *   1.实现方法签名
 *   2.实现方法默认行为
 *   3.trait 作为参数 (item: &impl Summary)
 *   4.trait bound：trait类型参数化
 *   5.通过+指定多个trait bound
 *   6.通过where简化trait bound
 *   7.返回实现了trait的类型
 *   8.使用trait bound 有条件地实现方法
 * 生命周期确保引用有效
 *   1.引用的生命周期
 *   2.生命周期注解语法：&i32; &'a i32; &'a mut i32;
 *   3.函数签名中的生命周期注解
 *     a.先声明生命周期注解参数
 */
// use generic_trait_and_lifetime::{Summary, Debug, Tweet};
// use std::fmt::Display;
// fn main() {
    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false,
    // };
    // println!("1 new tweet: {}", tweet.summarize());
    // notify(&tweet);
    // notify1(&tweet, &tweet);
// }

// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// fn notify1<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news! {}", item1.summarize());
//     println!("Breaking news! {}", item2.summarize());
// }

// fn notify2(item: &(impl Summary + Display)) {
//     println!("Breaking news! {}", item.summarize());
// }

// fn notify3<T: Summary + Display>(item: &T){}

// fn notify4<T: Summary + Clone, U: Clone + Debug>(t: &T, u: &U){}

// fn notify5<T, U>(t: &T, u: &U)
//     where T: Summary + Clone,
//           U: Clone + Debug
// {}

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
