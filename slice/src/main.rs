fn main() {
    //***文字列スライス***//
    //不変な借用の一種
    //従って、.clear()は使用できない
    //文字列の部分参照ができる
    {
        let s: String = String::from("hello");
        //先頭からindex-1まで
        let head_s: &str = &s[..1];
        //番号のindexから番号のindex-1まで
        let tail_s: &str = &s[s.len() - 1..];
        //番号のindex-1まで
        let inside_s: &str = &s[1 .. s.len() - 1];
        //全体を指し示す
        let all_s: &str =  &s[..];
        println!("s: {}, head_s: {}, inside_s: {}, tail_s: {}, all_s: {}", s, head_s, inside_s, tail_s, all_s);
    }

    //***その他のスライス***//
    {
        let num_arr: [i32; 5] = [1, 2, 3, 4, 5];
        let num_arr_slice: &[i32] = &num_arr[0..2];
        let num_arr_slice2: &[i32] = num_arr_slice;
        println!("num_arr_slice2: {}, {}", num_arr_slice2[0], num_arr_slice2[1]);
    }
}