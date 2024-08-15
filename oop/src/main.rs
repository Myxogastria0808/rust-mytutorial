fn main() {
    //***オブジェクト指向プログラミング***/
    {
        //*トレイトオブジェクト*//
        //*コンパイル時に型が決まっていなくてもよいトレイト境界
        trait Draw {
            fn draw(&self);
        }
        //Boxに入れることで、大きさを固定できる
        //Boxは、スタックにヒープへのポインタのデータしか持たないために、データサイズが決定する
        //impl ... コンパイル時に型が決まる
        //dyn .. コンパイル時に型が決まらない
        struct Screen {
            pub components: Vec<Box<dyn Draw>>, // <---> impl Draw
        }
    }

    {
        //*トレイトオブジェクトの使用例*//
        //*型を抽象化できる*//
        pub trait Draw {
            fn draw(&self) {}
        }

        //*ベクタの要素は、型が異なっていてもOK!
        //トレイトオブジェクト
        pub struct Screen1 {
            pub components: Vec<Box<dyn Draw>>,
        }

        //*ベクタの要素の型は、異なっていたらダメ！
        //ジェネリック型のトレイト
        pub struct Screen2<T: Draw> {
            pub components: Vec<T>,
        }

        //型の異なる2つのタプル構造体を作成する
        pub struct A(usize);
        pub struct B(i32);

        //A,B 共にDrawトレイトを実装する
        impl Draw for A {}
        impl Draw for B {}

        //ベクタ型の要素の型が違うのにエラーにならない
        let _screen1: Screen1 = Screen1 {
            components: vec![Box::new(A(1)), Box::new(B(2))],
        };
        //以下は、できない
        //let screen2: Screen2<A> = Screen2 { components: vec![A(1), B(2)], }
    }
}
