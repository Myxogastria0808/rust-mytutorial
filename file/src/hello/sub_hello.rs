pub fn call_area() -> () {
    use crate::hello::Area;
    let child_area: Area = Area {
        width: 10,
        height: 10,
    };
    println!("sub_helloモジュール");
    println!("{}", super::call_area(&child_area));
}

pub fn say_hello() -> () {
    println!("sub_helloモジュール");
    println!("Hello, World! from sub_hello module");
}