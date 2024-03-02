/*
 * 1.枚举enum：
 *  a.枚举的定义
 *    enum IP {
 *      V4,
 *      V6,
 *    }
 *  b.枚举成员的实例化
 *    let v4 = IP::V4, 成员的实例是IP枚举类型
 *  c.含数据的枚举类型的定义
 *    enum IP {
 *      V4(String),
 *      V6(String),
 *    }
 *  d.Option
 *    enum Option<T>{
 *      None,
 *      Some(T),
 *    }
 * 2.匹配match:
 *   a.通配模式other和占位符模式_ 
 *     i.other：涵盖所有其他可能值
 *     ii._:可以匹配任意值而不绑定值，可以使用空元组()作为_分支的代码，表示不做任何操作
 * 3.if let else 是match的语法糖
 */
fn main() {
    // let coin = Coin::Quarter;

    // match coin {
    //     Coin::Penny => 1,
    //     Coin::Nickel => 5,
    //     Coin::Dime => 10,
    //     Coin::Quarter => 25,
    // };
    let config_max: Option<i32> = Some(4);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // };
    if let Some(x) = config_max {
        println!("The maximum is configured to be {}", x);
    }
}

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// fn route(_ip_kind: IpAddrKind) {
    
// }

// enum MyEnum {
//     Variant1(i32),
//     Variant2(String),
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }