use std::path::is_separator;

#[derive(Debug)]

enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn show_this_message(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    {
        let mut message: Message = Message::Quit;
        message.show_this_message();
        message = Message::Move { x: 30, y: 40 };
        message.show_this_message();
        message = Message::Write("hello".to_string());
        message.show_this_message();
        message = Message::ChangeColor(255, 255, 255);
        message.show_this_message();
    }

    {
        //***Option***//
        //標準ライブラリで以下が定義されている
        // enum Option<T> {
        //     Some(T),
        //     None,
        // }

        let mut maybe_number: Option<i32> = Some(5);
        println!("{:?}", maybe_number);
        maybe_number = None;
        println!("{:?}", maybe_number);
    }

    {
        //***match式***//
        enum Color {
            Red,
            Green,
            Blue,
            Yellow,
        }

        fn color_to_str(color: &Color) -> &str {
            //RED #FF0000
            //Green #00FF00
            //Blue #0000FF
            //*matchは、対象のenumをすべてカバーする必要がある*//
            match color {
                Color::Red => "#FF0000",
                Color::Green => "#FF0000",
                Color::Blue => "#FF0000",
                Color::Yellow => "#FFFF00",
            }
        }

        let green: Color = Color::Green;
        println!("Green Color Code: {}", color_to_str(&green));
        let yellow: Color = Color::Yellow;
        println!("Yello Color Code: {}", color_to_str(&yellow));
    }

    {
        //***Optionとmatch***//
        //標準ライブラリで以下が定義されている
        // enum Option<T> {
        //     Some(T),
        //     None,
        // }
        fn find_myabe_number(maybe_number: Option<u32>) {
            match maybe_number {
                Some(number) => println!("found {}", number),
                None => println!("Nothing found."),
            }
        }

        find_myabe_number(Some(5));
        find_myabe_number(None);
    }

    {
        //***if let***//
        let maybe_number: Option<u32> = Some(3);
        //***matchの中の1つだけを問いだすときによい***//
        if let Some(number) = maybe_number {
            println!("number: {}", number);
        } else {
            println!("this is else statement.");
        }
    }

    {
        //***if letの練習***//

        enum Door {
            Open,
            Close,
        }

        let is_open: Door = Door::Open;

        match is_open {
            Door::Open => println!("ドアが開いている"),
            Door::Close => println!("ドアが閉まっている"),
        }

        //boolでやると...
        enum DoorBool {
            Open(bool),
        }

        let is_open_bool: DoorBool = DoorBool::Open(true);

        match is_open_bool {
            DoorBool::Open(true) => println!("ドアが開いている"),
            DoorBool::Open(false) => println!("ドアが閉まっている"),
        }

    }
}
