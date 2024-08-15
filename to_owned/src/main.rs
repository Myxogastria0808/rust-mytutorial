fn main() {
    // to_owned()
    // &の参照に対して使う
    // 参照もとの値をクローンする
    // 参考サイト: https://r9.hateblo.jp/entry/2022/06/07/213537
    {
        let s1: &str = "hello";
        let s2: &str = (&s1).to_owned();
        println!("s1: {}", s1);
        println!("s2: {}", s2);
    }
    {
        let int1: i32 = 10;
        //これをみるとよくわかるが、所有権がcloneされている
        //そのため、型が参照ではなくその型そのものになっている
        let int2: i32 = (&int1).to_owned();
        println!("int1: {}", int1);
        println!("int2: {}", int2);
    }
    {
        //vectorについて見てみると、
        //clone()していないのに所有権がコピーされている
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = (&v1[0..3]).to_owned();
        println!("v1: {:?}", v1);
        println!("v2: {:?}", v2);
    }
}
