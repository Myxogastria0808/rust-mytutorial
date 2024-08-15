fn main() {
    //***使用していない変数のwarningを消す方法***//
    //_(アンダーバー)を変数の先頭に付けると消える
    let _a: i32 = 1;
    //アンダーバーをなしでやると、以下のようにwarningが出る
    let b:i32 = 10;

    //enumも同じ
    enum Door {
        Open,
        _Close,
    }
    let is_open: Door = Door::Open;
    //***match文でも似たようなことができる***//
    //?if letを使うべきではある
    match is_open {
        Door::Open => println!("Open"),
        _ => println!("anything"),
    }

    //test
    println!("{}", tips::hello::say_hello());
    use tips::hello::say_hello;
    println!("{}", say_hello());


    {
        //Arataさんはこっち派
        enum Door {
            Open,
            _Close,
        }

        let mut is_open: Door = Door::Open;

        match is_open {
            Door::Open => println!("開いてるよ～"),
            Door::_Close => println!("閉まっているよ~"),
        }

        //**************************************** */
        //俺はこっちで書く
        enum DoorBool {
            Open(bool),
        }

        let mut is_open_bool: DoorBool = DoorBool::Open(true);

        match is_open_bool {
            DoorBool::Open(true) => println!("開いてるよ～"),
            DoorBool::Open(false) => println!("閉まっているよ~"),
        }

        //パターン2
        //却下
        enum DoorWindow {
            DoorOpen,
            DoorClose,
            WindowOpen,
            WindosClose,
        }

        let mut is_open_door_window: DoorWindow = DoorWindow::DoorOpen;

        if let DoorWindow::DoorOpen = is_open_door_window {
            println!("Door -> Open, Window -> ?");
        } else {
            println!("Door -> Close, Window -> ?");
        };

        //パターン3
        //これいい感じ
        enum OpenClose {
            Open, 
            Close,
            Hanbiraki,
        }

        enum Color {
            RGB(u8, u8, u8),
            HEX(String),
        }

        enum TypeDoor {
            Hikido,
            Husuma,
        }

        struct DoorS {
            is_open: OpenClose,
            color: Color,
            type_door: TypeDoor,
            key_locked: bool,
        }

        impl DoorS {
            fn change_key_state(&mut self) {
                self.key_locked = !self.key_locked
                // if self.key_locked {
                //     self.key_locked = false
                // } else {
                //     self.key_locked = true
                // }
            }
        }

        struct WindowS {
            is_open: OpenClose,
            color: Color,
            key_locked: bool,
        }

        //岡部
        enum DoorOkabe {
            Open,
            Close,
        }
        //************************** */
        struct DoorStruct {
            is_open: bool,
        }
    }
}
