fn main() {
    //スタック
    //メモリ近くに領域を確保
    //ヒープ
    //ガベージコレクション的なもの？

    //所有権
    //スコープの外では、sは破棄される
    {
        let s: &str = "hello";
        //OK
        println!("{}", s);
    }
    //NG
    //println!("{}", s);

    //**ヒープ領域における所有権の挙動***/
    //move (所有権の移動を意味する)
    let s1: String = String::from("Hello");
    let s2: String = s1;
    //この時点で、s1は値を保持していない
    println!("{}", s2);

    //clone(deep copyに当たる)
    //独立したデータとして保持する
    let s1_clone: String = String::from("hello, clone!");
    let s2_clone: String = s1_clone.clone();
    println!("s1_clone: {}\ns2_clone: {}", s1_clone, s2_clone);

    //***スタック領域のみに値が保存されている場合の所有権の挙動***//
    //値の全てがスタック領域にある場合は、毎回deep copyされている。
    let x: usize = 0;
    let mut y: usize = x;
    println!("x: {}, y: {}", x, y);
    y = 1;
    println!("x: {}, y: {}", x, y);

    //***ヒープ領域に保存されている場合とスタック領域に保存されている場合の違い***/
    {
        //ヒープ領域に値が保存される場合
        let s: String = String::from("hello");
        take_ownership(s); //ここでsがmoveされる

        //スタック領域に値が保存されている場合
        let x = 5;
        make_copy(x); //xは、Copyより、引き続き使用可能

        fn take_ownership(some_string: String) {
            println!("{}", some_string);
        }

        fn make_copy(some_integer: isize) {
            println!("{}", some_integer);
        }
    }
    //***所有権を返してあげるには? ***//
    //関数に渡した変数は、関数にスコープされるため、所有権が破棄される
    //呼び出し元関数にmoveし返している
    //s3にムーブされるといってもいいかもしれない
    {
        let s1: String = String::from("hello");
        let s2: String = takes_and_gives_back(s1);

        println!("s2: {}", s2);

        fn takes_and_gives_back(some_string: String) -> String {
            some_string
        }
    }
    //***参照(借用)をすればよい***//
    //***不変な借用***//
    {
        let s1: String = String::from("hello");
        let s1_len: usize = calculate_length(&s1);

        println!("s1: {}, s1_len: {}", s1, s1_len);

        fn calculate_length(s: &String) -> usize {
            let length = s.len();
            length
        }
    }
    //***可変な借用***//
    {
        let mut s: String = String::from("hello");
        change(&mut s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }

        println!("s: {}", s);
    }
    //***借用まとめ***//
    //mutにして、&mutで借用が可能になる
    //?一つしか可変な参照を持てない <- 競合が起きないようにするため
    //?NG例
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; <-これはできない！
    //?不変な参照と可変な参照を同時に持てない <- それはそうじゃないとやばいよね😊
    //?NG例
    //let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &mut s; <-これはできない！
    //?不変な参照は、同時に複数も持たせられる
    //?OKな例
    //let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s; <-これはできる！

    //***NG例: ダングリングポインタ (他人に渡されてしまった可能性のあるメモリを指すポインタのことであり、その箇所へのポインタを保持している間に、 メモリを解放してしまうこと)***//
    //let reference_to_nothing = dangle();
    //fn dangle() -> &String {
    //    let s: String = String::from("hello");
    //    &s <-sは、関数内でしか生きられないので、参照先が消える
    //}
    //***参照の規則***//    
    //1. 任意のタイミングで、一つの可変参照か不変な参照いくつでものどちらかを行える。
    //2. 参照は常に有効でなければならない。
}
