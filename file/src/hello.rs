//superを使うためにsubモジュールを持ってくる
pub mod sub_hello;

pub fn say_hello() -> () {
    println!("helloモジュール");
    let parent_area: Area = Area {
        width: 11,
        height: 2,
    };
    println!("Hello, World! from hello module");
    println!("{}", self::call_area(&parent_area));
}

struct Area {
    width: u32,
    height: u32,
}

fn call_area(area: &Area) -> u32 {
    area.width * area.height
}