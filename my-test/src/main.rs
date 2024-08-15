pub fn add_two_correct(num: i32) -> i32 {
    num + 2
}

pub fn add_two_false(num: i32) -> i32 {
    num + 3
}

pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn check_larger(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
}

//単体テストするときは、以下の記述を忘れない
#[cfg(test)]
mod tests {
    use super::*;

    //***testを行う際は、#[test]を記述する***//
    //***cargo test で、テストが走る***//
    //*assert_eq!()*//
    #[test]
    fn test1() {
        let a: i32 = add_two_correct(23);
        //第一引数と第二引数が等しいときに、正しいと判定される
        assert_eq!(25, a);
    }

    #[test]
    //あえて、panic!していることが正しいとしたいときは、以下を追加する
    //メッセージも書ける
    #[should_panic(expected = "This msg is shoud_panic msg!!!")]
    fn test2() {
        let a: i32 = add_two_false(23);
        assert_eq!(25, a);
    }

    //*assert!()*//
    #[test]
    fn test3() {
        let rec_self: Rectangle = Rectangle {
            width: 20,
            height: 20,
        };
        let rec_other: Rectangle = Rectangle {
            width: 10,
            height: 10,
        };
        //trueのとき、正しいと判断される
        assert!(rec_self.check_larger(&rec_other));
    }

    #[test]
    fn test4() {
        let rec_self: Rectangle = Rectangle {
            width: 20,
            height: 20,
        };
        let rec_other: Rectangle = Rectangle {
            width: 10,
            height: 10,
        };
        //失敗メッセージを後に実装できる
        assert!(
            rec_other.check_larger(&rec_self),
            "The value is: {}. You have to return true!", rec_other.check_larger(&rec_self)
        );
    }

    //*Result<T, E>も使える！*//
    //Ok(T) -> ok
    //Err(F) ->  FAILED
    #[test]
    fn test5() -> Result<(), String> {
        if 2 + 2  != 4 {
            Ok(())
        } else {
            Err(String::from("This is a error message🐈"))
        }
    }

}