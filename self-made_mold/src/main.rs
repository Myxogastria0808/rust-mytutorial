pub mod my_type;
use crate::my_type::Guess;

fn main() {
    let a: Guess = Guess::set_value(100);

    println!("a: {:?}", a);
    println!("a: {}", a.get_value());
}
