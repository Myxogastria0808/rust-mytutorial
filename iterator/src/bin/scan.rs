//参考: https://qiita.com/hystcs/items/f7f40026bb34421a1271
fn main() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //*scan(初期値, |現在の状態(mutな参照　<-値の変更には参照外しが必要), 次の要素(イテレーターに対してnextメソッドを読んだ際の返り値 <- Optionメソッドでラップされている)| {現在の状態と次の状態についての処理; Some(結果)})
    let mut cumsum: Vec<u32> = v
        .iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect();

    //先頭に10を追加
    cumsum.insert(0, 0);

    println!("{:?}", cumsum);
}
