pub mod sub_hello;

pub fn say_hello() -> String {
    "Hello, World!".to_string()
}

fn call_say_hello() -> () {
    println!("{}", self::say_hello());
}