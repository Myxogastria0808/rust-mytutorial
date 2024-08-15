fn main() {
    //***回復不可能なエラー***//
    //***panic!("メッセージ")***//
    //panic!("Error: HelloWorld!");

    //*RUST_BACKTRACE=1 cargo runをすると、詳細情報が見れる

    //***回復なエラー***//
    //Resultの定義
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    use std::fs::File;
    use std::io::ErrorKind;
    use std::io;
    use std::io::Read;
    {
        //使用例1
        let f: Result<File, std::io::Error> = File::open("hello.txt");
        let _f: File = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("There was a problem opening the files: {:?}", error);
            }
        };
    }
    {
        let f: Result<File, std::io::Error> = File::open("hello.txt");

        let f: File = match f {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(file_create) => file_create,
                    Err(e) => {
                        panic!("Tried to create file but there was a problem: {:?}", e)
                    }
                }
            },
            Err(error) => {
                panic!("There was a problem opening the file: {:?}", error)
            },
        };
    }

    {
        //***エラー時にパニックするショートカット***//
        //*unwrap*//
        //ResultがOk列挙子なら、unwrapはOkの中身を返す。
        //ResultがErr列挙子なら、 unwrapはpanic!を呼ぶ。
        let f1: File = File::open("hello.txt").unwrap();
        //*expect*//
        //Result値がOk列挙子なら、expectはOkの中身を返す。
        //ResultがErr列挙子なら、 expectはpanic!を呼ぶ。正し、エラーメッセージは、自分で設定できる。
        let f2: File = File::open("hello.txt").expect("Error!!!!!!!!!!!!!!");
    }

    {
        //***戻り値がResult<T, E>の関数の処理***//
        //リファクタリング前の関数の状態
        fn read_username_from_file() -> Result<String, io::Error> {
            let f: Result<File, io::Error> = File::open("hello.txt");

            let mut f: File = match f {
                Ok(file) => file,
                Err(e) => return  Err(e),
            };

            let mut s: String = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        //リファクタリング1
        //*Resultの場合は、matchを?で省略できる!//
        fn read_username_from_file_ref1() -> Result<String, io::Error> {
            let mut f: File = File::open("hello.txt")?;
            let mut s: String =String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        //リファクタリング2
        //メソッドチェインを作ることができる！
        fn read_username_from_file_ref12() -> Result<String, io::Error> {
            let mut s: String = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
}
