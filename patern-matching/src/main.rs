fn main() {
    {
        //***match文***//
        let x: u8 = 1;
        match x {
            1 | 2 => println!("one or two"),         //1 or 2
            3..=5 => println!("three through five"), //1 から 5 = 1 | 2 | 3 | 4 | 5
            _ => println!("anything"),               //1, 2, 3以外なんでも
        }

        //*charで面白いことができる*//
        let y: char = 'a';
        match y {
            'a'..='c' => println!("a, b or c"), //数値型とchar型で利用可能
            _ => println!("anything"),
        }
    }

    {
        //***構造体を展開する的なこと***//
        struct Point {
            x: i32,
            y: i32,
        }

        let p: Point = Point { x: 0, y: 7 };

        let Point { x, y } = p;
        println!("x: {}, y: {}", x, y);
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }

        //*構造体でmatch文を使う*//
        let p: Point = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("y = 0"),
            Point { x: 0, y } => println!("x = 0"),
            Point { x, y } => println!("x != 0, y != 0, x = {}, y = {}", x, y),
        }
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(u8, u8, u8),
        }

        //*enumでmatch文を使う*//
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }

    {
        //***if let文***//
        //*if let文は、組み合わせることができる*//
        let favorite_color: Option<&str> = None;
        let is_tuesday: bool = false;
        let age: Result<i32, String> = Ok(35);

        println!("\n");
        println!("if let文");
        if let Some(color) = favorite_color {
            println!("favorite_color is : {}", color);
        } else if is_tuesday {
            println!("favorite color is None! Today is Tuesday!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("age > 30, age is {}", age);
            } else {
                println!("age <= 30, age is {}", age);
            }
        } else {
            println!("favorite color is None! Today isn't Tuesday! Age is Err(String)");
        };

        //***let else文***//
        println!("\n");
        println!("let else文");
        let Ok(age) = age else {
            println!("age is {:?}", age);
            return;
        };
    }

    {
        //***while let条件分岐ループ***//
        println!("\n");
        println!("while let条件分岐ループ");

        let mut stack: Vec<i32> = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("top: {}", top);
        }
    }

    {
        //***要素が構造体のイテラルで、構造体のフィールドの内容を変化させる方法***//
        println!("\n");
        println!("要素が構造体のイテラルで、構造体のフィールドの内容を変化させる方法");

        struct Point {
            x: i32,
            y: i32,
        }

        let points: Vec<Point> = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];

        //mapで展開を行っている
        let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        println!("sum_of_squares: {}", sum_of_squares);
    }

    {
        //*refとref mutについて*//
        //*matchの文脈において、&は差参照の値にマッチする役割があり、
        //*&を付けても、参照を生成しない！
        //*そこで、refとref mutを用いた参照を行う
        {
            //refを付けないとmoveされてしまう💀
            let robot_name: Option<String> = Some(String::from("Bors"));
            match robot_name {
                Some(name) => println!("Found a robot name: {}", name),
                None => (),
            }
            //これは、エラーになる (moveされたため)
            //println!("robot_name: {:?}", robot_name);
        }
        {
            //* refを付けたパターン
            let robot_name: Option<String> = Some(String::from("Bors"));
            match robot_name {
                Some(ref name) => println!("Found a robot name: {}", name),
                None => (),
            }
            println!("robot_name: {:?}", robot_name);
        }
        {
            //* ref mutを付けたパターン
            let mut robot_name: Option<String> = Some(String::from("Bors"));
            match robot_name {
                Some(ref mut name) => *name = String::from("Another name"),
                None => (),
            }
            println!("robot_name: {:?}", robot_name);
        }
    }

    {
        //***無視するキーワード _ ..***//
        println!("\n");
        println!("無視するキーワード _ ..");
        let numbers: (i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5);

        //* _ の例 *//
        fn ignore_even_number(first: i32, _: i32, third: i32, _: i32, five: i32) -> () {
            println!("first: {}, third: {}, five: {}", first, third, five);
        }
        ignore_even_number(numbers.0, numbers.1, numbers.2, numbers.3, numbers.4);

        //* .. の例 */
        match numbers {
            (first, .., last) => {
                println!("first: {}, last: {}", first, last);
            }
        }
    }

    {
        //***追加の条件式***//
        //*match文では、追加の条件式を書くことができる*//
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("x >= 5, x: {}", x),
            None => (),
        }

        //*関係のない変数を条件式として混ぜることができる*//
        let num2: Option<i32> = Some(4);
        let y: bool = false;

        match num2 {
            Some(x) if y => println!("y is true!, x: {}", x),
            Some(x) => println!("y is false!, x: {}", x),
            None => (),
        }
    }
}
