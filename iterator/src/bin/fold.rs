//参考: https://qiita.com/hystcs/items/f7f40026bb34421a1271
fn main() {
    let v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //* fold(初期値, |現在の状態, 次の要素| 現在の状態と次の状態についての処理)
    //foldは、一つのの値を返す
    let sum = v.iter().fold(0, |sum, x| sum + x);
    println!("{}", sum);
}
