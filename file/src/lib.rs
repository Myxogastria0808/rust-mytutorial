//ここにmodを集める
pub mod hello;

pub fn lib_func() {
    print!("以下、lib.rs(lib_funcの実行結果)の実行結果");
    self::hello::say_hello();
    self::hello::sub_hello::call_area();
}