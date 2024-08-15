fn main() {
    {
        //TODO 標準ライブラリのstructについて見ていく
        //***ベクタ***//
        //参考資料
        //https://docs.rs/rustc-std-workspace-std/latest/std/vec/index.html
        //*ベクタの生成*//
        //ベクタの生成 パターン1
        let _v1: Vec<i32> = Vec::new();
        //ベクタの生成 パターン2
        let _v2: Vec<i32> = vec![1, 2, 3];

        //*ベクタ更新*//
        let mut v3: Vec<i32> = Vec::new();
        v3.push(1);
        v3.push(2);
        v3.push(3);

        //*ベクタの要素を読む*//
        let v_sample: Vec<i32> = vec![1, 2, 3, 4, 5];

        //パターン1 (良くないパターン)
        //*これは、ベクタのインデックスをはみ出ても、コンパイルエラーになるまで気が付きにくい */
        let third: &i32 = &v_sample[2];
        println!("third: {}", third);

        //パターン2 (推奨！)
        //*これは、存在するかどうかをOptional<T>の型として取得できる */
        //?プログラムが終了することがないので、良い！
        match v_sample.get(2) {
            Some(third) => println!("Third: {}", third),
            None => println!("The index is out of range."),
        }

        // {
        //     以下は、不変な参照をしているベクタをに要素追加するためにエラーになる
        //     let mut bad_v: Vec<i32> = vec![1, 2, 3, 4, 5];
        //     let first: &i32 = &bad_v[0]; //ここで不変な参照をしている
        //     そのため、以下がエラーになる
        //     bad_v.push(1); //firstが可変な変数であれば大丈夫
        // }

        //*ベクタの要素を読む*//
        let read_v: Vec<i32> = vec![100, 32, 57];
        for i in &read_v {
            println!("{}", i);
        }

        //*ベクタの要素を変化させる*//
        let mut change_v: Vec<u32> = vec![1, 2, 3];
        for i in &mut change_v {
            //参照外し演算子 *変数名 を使用する
            *i = i.pow(2u32);
        }
        for i in &change_v {
            println!("{}", i);
        }

        //*複数の型のベクタ*//
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let _row: Vec<SpreadsheetCell> = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    {
        //***String***//
        //***Stringは、Vec<u8>のラッパー <-これはかなり重要 */
        //参考資料
        //https://doc.rust-lang.org/std/string/struct.String.html
        //*Stringの生成*//
        let mut _gen_s: String = String::new();

        //*Stringへの変換*//
        //パターン1
        let _change_s1: String = "hello".to_string();
        //パターン2
        let _chnage_2: String = String::from("hello");

        //*文字列の結合*//
        //*&str型 */
        //&str型の文字列を追加する。
        let mut join_str1: String = String::from("he");
        let join_str2: &str = "llo";
        join_str1.push_str(join_str2);
        println!("join_str1: {}", join_str1);
        println!(
            "join_str2: {} <- つまり、push_str()は、join_str2を参照していた",
            join_str2
        );

        //*char型 */
        //char型の文字列を追加する。
        let mut join_char1: String = String::from("lo");
        let join_char2: char = 'l';
        join_char1.push(join_char2);
        println!("join_char1: {}", join_char1);
        println!(
            "join_char2: {} <- つまり、push()、join_char2を参照していた",
            join_char2
        );

        //String型の文字列同士の結合
        //*String型 */
        //*非推奨
        let join_string1: String = String::from("Hello, ");
        let join_string2: String = String::from("World!");
        let joined_string3: String = join_string1 + &join_string2;
        println!("joined_string3: {}", joined_string3);
        println!(
            "join_string2: {} <- join_string2は、参照だからできる",
            join_string2
        );
        //*join_string1は、moveされているため、もう存在しない！！！！
        //*以下のコードは、エラーとなる
        //*println!("join_string1: {}", join_string1);

        //*推奨
        let join_string21: String = String::from("tik");
        let join_string22: String = String::from("tak");
        let join_string23: String = String::from("toe");
        //*formatマクロは、String型を返す!*//
        //*そして、全ての変数の所有権を奪わない*// <-これが非常に重要！！！！
        let joined_string2123: String =
            format!("{}{}{}", join_string21, join_string22, join_string23);
        println!(
            "join_string21: {} <- join_string21は、参照だからできる",
            join_string21
        );
        println!(
            "join_string22: {} <- join_string22は、参照だからできる",
            join_string22
        );
        println!(
            "join_string23: {} <- join_string23は、参照だからできる",
            join_string23
        );
        println!(
            "joined_string2123: {} <- join_string2123は、参照だからできる",
            joined_string2123
        );

        //*文字列へのアクセス*//
        //*Stringは、Vec<u8>のラッパ (重要) */
        //バイトの長さは、len()でできる
        let get_len: usize = String::from("Hola").len();
        let get_len_jp: usize = String::from("こんにちは").len();
        println!(
            "{} = 4 (Hola) 必ずしも、文字列の長さにはなっていない！",
            get_len
        );
        println!(
            "{} ≠ 5 (こんにちは) 必ずしも、文字列の長さにはなっていない！",
            get_len_jp
        );
        //従って、Rustではindexを用いたアクセス(String型の変数[index])が禁止されている
        //*文字列のスライスは、できる! (ただ。文字数で取り出せないのでpanicを起こす可能性がある)
        let hello: String = "こんにちは".to_string();
        let hello_slice = &hello[0..6];
        println!("hello_slice: {}", hello_slice);

        //***一文字ずつ取り出すには。以下の方法を用いる***//
        for c in "こんにちは".chars() {
            println!("{}", c);
        }
        //***各byteをそのまま返す***//
        for b in "こんにちは".bytes() {
            println!("{}", b);
        }
    }

    {
        //***HashMap (連想配列のこと)***//
        //*HashMapを使う際には、必ずuseをする必要がある！ */
        use std::collections::HashMap;
        //*HashMapの生成*//
        let mut scores: HashMap<String, i32> = HashMap::new();

        let teams: Vec<&str> = vec!["Blue", "yellow"];
        let initial_scores: Vec<i32> = vec![10, 50];

        let zip_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("zip_scores: {:?}", zip_scores);
        println!("teams: {:?}", teams);
        println!("initial_scores: {:?}", initial_scores);

        //*HashMapの要素追加*//
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 50);

        println!("scores: {:?}", scores);

        //*insertの注意点*//
        //*使用した変数は、insertメソッドによってmoveされてしまう!
        let key_ex: &str = "Hello";
        let value_ex: i32 = 10;
        let mut hello_hash: HashMap<&str, i32> = HashMap::new();
        hello_hash.insert(key_ex, value_ex);

        println!("hello_hash: {:?}", hello_hash);

        //*Hashmapの値の更新*//
        //*同じキーに異なる値を挿入すればいい */
        scores.insert(String::from("Blue"), 10);

        println!("scores: {:?}", scores);

        //*キーに値がなかったときのみ値を挿入する*//
        let mut insert_hash: HashMap<String, u8> = HashMap::new();
        insert_hash.insert(String::from("White"), 0);

        //*entry(key).or_insert(value)で、できる */
        insert_hash.entry(String::from("White")).or_insert(0);
        insert_hash.entry(String::from("Blue")).or_insert(100);

        println!("insert_hash: {:?}", insert_hash);

        //*HashMapへのアクセス*//
        //値を取り出す方法
        struct Color(u8, u8, u8);
        let mut access_hash: HashMap<String, Color> = HashMap::new();
        access_hash.insert(String::from("Blue"), Color(255, 0, 0));
        access_hash.insert(String::from("yello"), Color(255, 255, 0));

        //取り出したい値のキーをgetメソッドに参照渡しする
        let color_name: String = String::from("Blue");
        let rgb: Option<&Color> = access_hash.get(&color_name);
        match rgb {
            Some(rgb) => println!(
                "color_name: {}, RGB: {}, {}, {}",
                color_name, rgb.0, rgb.1, rgb.2
            ),
            None => println!("This Hashmap has not the color RGB value."),
        }

        {
            //使用例
            println!("\n");
            let text: &str = "hello world wonderful world";
            let mut map: HashMap<&str, i32> = HashMap::new();

            //*split_whitespace()メソッド */
            let text_split = text.split_whitespace();
            println!("text_split: {:?}", text_split);

            for word in text.split_whitespace() {
                let count: &mut i32 = map.entry(word).or_insert(0);
                *count += 1;
            }
            println!("map: {:?}", map);
        }
    }
}
