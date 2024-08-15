#[derive(Debug)]

struct Sample {
    username: String,
    age: u32,
}

fn main() {
    //Debugのモードで、構造体の中身を生一気に確認できる
    let user1: Sample = Sample {
        username: String::from("Hello"),
        age: 20,
    };

    //#[derive(Debug)]をすることによって、{:?}でストラクトの中身を表示できる
    println!("{:?}", user1);
}
