fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //クロージャは、型推論される
    let add_one_v3 = |x| x + 1;
    println!("add one result v1: {}", add_one_v1(1));
    println!("add one result v2: {}", add_one_v2(1));
    println!("add one result v3: {}", add_one_v3(1));

    {
        //環境をキャプチャできる
        let x: i32 = 4;
        //クロージャはできる
        //ｘの参照をしている
        let equal_to_x = |z| z == x;
        //関数ではできない
        //fn equal_to_x(z: i32) -> bool {z == x}
        let y: i32 = 4;
        println!("equal to x: {}", equal_to_x(y));
        // 以下は、エラーとならない
        // xが存在するため
        println!("x: {:?}, equal to x: {}", x, equal_to_x(y));
    }

    {
        //環境をキャプチャするときの注意
        //moveキーワードの使用をすると、所有権が移動する
        //スタックに保存される方は、Copyトレイトが実装されているので、問題なし
        let x: Vec<i32> = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;
        let y: Vec<i32> = vec![1, 2, 3];
        println!("equal to x: {}", equal_to_x(y));
        //以下は、エラーとなる
        //xが存在しないため
        //println!("x: {:?}, equal to x: {}", x, equal_to_x(y));
    }
}
