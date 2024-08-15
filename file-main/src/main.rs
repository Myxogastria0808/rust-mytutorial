//モジュールを使う宣言
pub mod second;
pub mod third;

//使用する宣言
use crate::second::hello;
use crate::third::return_three::return_three;

fn main() {
    hello();
    println!("return_three: {}", return_three());
}
