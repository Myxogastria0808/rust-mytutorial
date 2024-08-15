//ここにmodを集めることができる
pub mod hello;


use crate::hello::say_hello;

fn hoge() {
    println!("hoge");
    self::hello::say_hello();
}