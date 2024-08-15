fn main() {
    {
        //不変参照は、シャドーイングによって値の更新ができる
        let x = 10;
        let borrowed_x = &x;
        println!("x: {}", x);
        println!("borrowed_x: {:?}", borrowed_x);
        let x = 0;
        let borrowed_x = &x;
        println!("x: {}", x);
        println!("borrowed_x: {:?}", borrowed_x);
    }

    {
        //可変参照
        let mut x: i32 = 10;
        let borrowed_x: &mut i32 = &mut x;
        // println!("x: {}", x); <-可変参照の場合は、そもそも参照先の値を利用できない
        println!("borrowed_x: {:?}", borrowed_x);
        *borrowed_x = 0; //<-参照外しをした可変参照は、値の変更が可能 (mutableになっている)
        println!("borrowed_x: {}", borrowed_x);
    }
}
