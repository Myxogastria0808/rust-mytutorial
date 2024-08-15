fn main() {
    //***変数宣言***//
    //再代入可能
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //再代入不可能
    let y: i32 = 1;
    println!("The value of y is: {}", y);
    //NG
    //y = 2;
    //コンパイル時点で値が入っている必要がある
    const CONST: i32 = 100;
    println!("The value of CONST is: {}", CONST);

    //シャドーイング(同じ変数を何度も宣言できることをシャドーイングという)
    let y: i32 = 5;
    let y: i32 = y * 10;
    println!("The value of y is: {}", y);

    //ブロックでスコープする
    {
        let y: i32 = 10;
        println!("The value of inner y is: {}", y);
    }
    println!("The value of y is: {}", y);

    //シャドーイングは再宣言より、違う型でもできる！
    let some_strings: &str = "aaa";
    println!("The value of some_strings is: {}", some_strings);
    let some_strings: usize = some_strings.len();
    println!("The value of some_strings is: {}", some_strings);
    //再代入はできない！
    //NG
    // let mut some_strings: &str = "aaa";
    // println!("The value of some_strings is: {}", some_strings);
    // let some_strings: usize = some_strings.len();
    // println!("The value of some_strings is: {}", some_strings);

    //***データ型***//
    //整数型
    //符号付き ... i
    //符号なし ... u
    //8, 16, 32, 64bit がそれぞれ存在
    //各アーキテクチャに最適化 isize, usize
    let i_sample: isize = -10;
    let u_sample: usize = 10;
    println!("The value of i_sample is: {}", i_sample);
    println!("The value of u_sample is: {}", u_sample);

    //リテラル
    //10進数 ... そのままで、_(アンダーバー)の混入を許す
    //ex)98_22
    //16進数 ... 0x~
    //ex) 0xff
    //8進数 ... 0o~
    //ex) 0o77
    //2進数 ... 0b~
    //ex) 0b1111_0000
    //バイト(u8のみ) ... b'文字'
    //ex) b'A'

    //浮動小数点型
    //f32, f64
    //*f64を使え！ */
    let f_sample: f64 = 3.141592;
    println!("The value of f_sample is: {}", f_sample);

    //四則演算
    //+, -, *, /, %
    //*同じ型である必要がある！ */
    //NG
    // let hello_1: usize = 6;
    // let hello_2: isize = 7;
    // let hello_12_result = hello_1 + hello_2;

    //一文字の型
    //char(UTF-8)
    let char_sample: char = '🐈';
    println!("The value of char_sample is: {}", char_sample);

    //複合型
    //タプル型
    //複数の型を一つの複合型にまとめる
    let tup_sample: (isize, usize, char) = (-10, 10, 'あ');
    //中身へのアクセス
    let tup_sample_1: isize = tup_sample.0;
    let tup_sample_2: usize = tup_sample.1;
    let tup_sample_3: char = tup_sample.2;
    println!("The value of tup_sample is: {}, {}, {}", tup_sample_1, tup_sample_2, tup_sample_3);

    //分割代入できる
    let tup_sample2: (isize, f64, char) = (-1, 3.14, 'あ');
    let (tup_sample_x, tup_sample_y, tup_sample_z) = tup_sample2;
    println!("The value of tup_sample2 is: {}, {}, {}", tup_sample_x, tup_sample_y, tup_sample_z);

    //配列型
    //固定長で、全て同じ型である必要がある。
    //スタック領域に保存される(ヒープではなく)
    let arr_sample: [u32; 3] = [1,2,3];
    println!("The value of arr_sample is: {}, {}, {}", arr_sample[0], arr_sample[1], arr_sample[2]);
    //配列の初期化
    //let 配列名: [型; 配列数] = [初期価値; 配列数];
    let arr_sample_init: [u32; 3] = [0; 3];
    println!("The value of arr_sample_init is: {}, {}, {}", arr_sample_init[0], arr_sample_init[1], arr_sample_init[2]);

    //関数
    say_unko();
    println!("{}", calc(10, 5));

    //式(ブロック全体の値は、最後のセミコロンをつかなかった時の値)
    //*これすごい！ */
    let formula_y = {
        let x = 3;
        x + 1
    };
    println!("The value of formula_y is: {}", formula_y);

    //String型
    //? &strは、入れられた値を見ることしかできない
    //文字列の結合なのでができるのが、String型
    //Sting型は、ヒープ領域
    let mut str_sample: String = String::from("hello");
    str_sample.push_str(", world");
    println!("{}", str_sample);
}

//関数
fn say_unko () {
    println!("unko!");
    return
}

fn calc (x: usize, y: usize) -> usize {
    let z: usize = x * y;
    return z
}