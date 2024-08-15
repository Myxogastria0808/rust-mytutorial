use std::fmt::Display;

fn main() {
    fn sample(a: i32) -> i32 {
        a * a
    }

    let a = 1;
    println!("sample result: {}", sample(a));

    struct Abc {
        username: String,
        nickname: String,
        age: f64,
    }

    impl Abc {
        fn say_username(&self) -> () {
            println!("username: {}", self.username);
        }
        fn say_nickname(self) -> () {
            println!("nickname: {}", self.nickname);
        }
    }

    let user1: Abc = Abc {
        username: String::from("Hello"),
        nickname: String::from("World"),
        age: 20f64,
    };

    {
        use core::ops::Add;

        struct PrintAny<T>
        where
            T: Add<Output = T> + Display + Copy,
        {
            print_object: T,
        }

        impl<T> PrintAny<T>
        where
            T: Add<Output = T> + Display + Copy,
        {
            fn print_any(&self, object: T) -> () {
                let a: T = self.print_object + object;
                println!("self.print_object + object: {}", a);
            }
        }

        let a: PrintAny<i32> = PrintAny { print_object: 123 };
        a.print_any(10);
    }
}
