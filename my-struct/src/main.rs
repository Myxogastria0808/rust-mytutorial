//***構造体***//
struct User {
    username:  String,
    email: String,
    sign_in_count: u64,
    activate: bool,
}

fn main() {
    let user1: User = User {
        email: String::from("hello@example.com"),
        username: String::from("hello world"),
        activate: true,
        sign_in_count: 1,
    };

    println!("user1");
    println!("email: {}", user1.email);
    println!("username: {}", user1.username);
    println!("activate: {}", user1.activate);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!("\n");

    //mutにすると、後で変更できる！
    let mut user2: User = User {
        email: String::from("hello2@example.com"),
        username: String::from("hello univserse"),
        activate: false,
        sign_in_count: 2,
    };

    println!("user2");
    println!("email: {}", user2.email);
    println!("username: {}", user2.username);
    println!("activate: {}", user2.activate);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("\n");

    //変更してみる
    user2.activate = true;

    println!("user2");
    println!("email: {}", user2.email);
    println!("username: {}", user2.username);
    println!("activate: {}", user2.activate);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("\n");

    //インスタンスの生成関数を作成した際のTips
    fn create_user1(email: String, username:String) -> User {
        User {
            email: email,
            username: username,
            activate: true,
            sign_in_count: 0,
        }
    }
    //以下のようにも記述できる
    fn create_user2(email: String, username: String) -> User {
        User {
            email,
            username,
            activate: true,
            sign_in_count: 0,
        }
    }

    //***タプル構造体***//
    struct Color (i32, i32, i32);

    let color1: Color = Color(255, 255, 255);
    println!("Color");
    println!("{}, {}, {}", color1.0, color1.1, color1.2);
    println!("\n");

    {
        //新しいインスタンスの生成時に、すでにあるインスタンスの値を引き継げる
        //足りていない分を引っ張ってくる
        struct Unko {
            user: String,
            color: (u32, u32, u32),
            elasticity: u32,
            sex: String,
        }

        let unko1: Unko = Unko {
            user: String::from("user1"),
            color: (255, 255, 255),
            elasticity: 12,
            sex: String::from("male"),
        };

        let unko2: Unko = Unko {
            user: String::from("user2"),
            sex: String::from("female"),
            //この記述でunko1から継承できる
            //Copy trait機能を持っている方しかできない。
            ..unko1
        };

        println!("unko1");
        println!("user: {}", unko1.user);
        println!("color: {}, {}, {}", unko1.color.0, unko1.color.1, unko1.color.2);
        println!("elasticity: {}", unko1.elasticity);
        println!("sex: {}", unko1.sex);
        println!("\n");

        println!("unko2");
        println!("user: {}", unko2.user);
        println!("color: {}, {}, {}", unko2.color.0, unko2.color.1, unko2.color.2);
        println!("elasticity: {}", unko2.elasticity);
        println!("sex: {}", unko2.sex);
        println!("\n");
    }

    //使えるかの確認
    let user3: User = create_user1("user3@example.com".to_string(), "user3".to_string());
    let user4: User = create_user2("user4@example.com".to_string(), "user4".to_string());
    
    println!("user3");
    println!("email: {}", user3.email);
    println!("username: {}", user3.username);
    println!("activate: {}", user3.activate);
    println!("sign_in_count: {}", user3.sign_in_count);
    println!("\n");

    println!("user4");
    println!("email: {}", user4.email);
    println!("username: {}", user4.username);
    println!("activate: {}", user4.activate);
    println!("sign_in_count: {}", user4.sign_in_count);
    println!("\n");

    //***構造体の使用例***//
    let mut triangle1: Triangle = Triangle {
        height: 10,
        bottom: 10,
    };

    fn calc_triangle_area(triangle: &Triangle) -> u32 {
        //以下で所有権を消費していないので、借用すべき
        triangle.height * triangle.bottom / 2
    }

    //実行
    let result1: u32  = calc_triangle_area(&triangle1);
    println!("triangle area is {}", result1);
    println!("\n");

    //関数は、借用で値を受け取るために、triangle1の値の変更は可能
    triangle1.bottom = 1;
}

struct Triangle {
    height: u32,
    bottom: u32,
}