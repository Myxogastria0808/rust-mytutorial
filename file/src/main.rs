use file::lib_func;

fn main() {
    //実用的なファイル分割の運用
    println!("以下、main.rsでの実行結果");
    file::hello::say_hello();
    file::hello::sub_hello::call_area();
    //以下の記述でもできる
    //このように書くことで、hello moduleのsay_helloとsub_hello moduleのsay_helloを区別できる
    use file::hello::*;
    println!("\n");
    say_hello();
    sub_hello::say_hello();
    sub_hello::call_area();

    //lib.rsで定義したやつの実行結果
    //lib.rsの関数は、そのまま使える
    println!("\n");
    lib_func();


}
