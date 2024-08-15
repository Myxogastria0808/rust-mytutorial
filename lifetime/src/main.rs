fn main() {
    //**ライフライム***//
    //***ライフタイムは、エラーが出たら考える***//
    //*関数でライフタイムを考える*//
    //?戻り値のみに、ライフタイム注釈を与えるとエラーになる
    //?理由) 戻り値なのに、関数のスコープから抜けると破棄されるため
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    //ライフタイムの符号を書かないと以下のようにコンパイラは、解釈してしまい、エラーになる。
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a or &'b str {　<-ここで、戻り値のライフタイムが明確でないので、エラーになる
    //     処理内容
    // }

    {
        let nyoki1: &str = "Hello!";
        let nyoki2: &str = "abc";
        println!("largest is: {}", longest(nyoki1, nyoki2));
    }

    //***structでライフタイムを使用する***//
    #[derive(Debug)]
    struct SomeStruct<'a> {
        _part: &'a str,
    }

    {
        let s: &str = "HEllo, world!";
        //some_struct1を宣言したところのスコープ範囲内のみ使用できる
        let some_struct1: SomeStruct = SomeStruct {
            _part: s,
        };
        println!("{:?}", &some_struct1); //<-これはできる
    }
    //ここは、some_struct1が破棄されているので、some_struct1は、見れない
    // println!("{:?}", &some_struct1);

    //***ジェネリックな型引数・トレイト境界・ライフタイム***//
    use std::fmt::Display;
    //ライフタイム, ジェネリックな型引数 の順に記述する
    fn longest_with_an_announcment<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let x: &str = "hello";
    let y: &str = "hello world";
    let z: String = String::from("This mold is String");
    let announce: &str = longest_with_an_announcment(x, y, z);
    println!("{}", announce);
}
