fn main() {
    let x: usize = 10;
    //if
    //条件式は、必ずbool型
    //返す型は、すべて同じ型にしなければいけない
    let y: &str = {
        if x > 10 {
            "OK"
        } else if x == 10 {
            "BAD"
        } else {
            "ERROR"
        }
    };
    println!("The value of y is: {}", y);

    //loop
    let mut count: usize = 0;
    //二重のloopでも、ラベルを指定して一気にbreakできる
    'counting_up: loop {
        loop {
            println!("count = {}", count);
            if count == 2 {
                break 'counting_up;
            }
            count += 1;
        }
    }

    //while
    let mut number = 3;
    while number > 0 {
        println!("{}", number);
        number -= 1
    }
    println!("発射！");

    //for
    let arr_sample: [usize; 5] = [10, 20, 30, 40, 50];
    for e in arr_sample {
        println!("the value is: {}", e);
    }
    //連番の書き方
    //n..m (n ~ m-1)
    for e in 0..5 {
        println!("the value is: {}", e);
    }

    //enumerateを使用したパターン
    println!("enumrateの出力結果");
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
