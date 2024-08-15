use std::{cell::BorrowError, rc::Rc};

fn main() {
    {
        //参照外しの復習
        let a: Vec<i32> = vec![1, 2, 3];
        //ここで、借用をしているが、参照外しによってtrueとなっている
        //参照外しは、借用からmoveに変えること
        let borrow_a: &Vec<i32> = &a;
        let b: Vec<i32> = vec![1, 2, 3];
        println!("erualy: {}", (b == *borrow_a));

        let mut moved_a: Vec<i32> = a; //move
        let muttably_borrow_a: &mut Vec<i32> = &mut moved_a;
        let b: Vec<i32> = vec![1, 2, 3];
        println!("erualy: {}", (b == *muttably_borrow_a));
    }

    {
        //***Box***//
        //*データをヒープにおいて、アドレスの情報だけを持っている*//
        //使い方
        let b: Box<i32> = Box::new(5);
        //本来は、スタック領域に保存されるべきデータがヒープに保存され、その参照のデータをbが格納している
        //使うときは、普通に使える
        println!("b = {}", b);

        //***実際の使用方法***//
        //*ボックスで再帰的な型を可能にする*//
        //以下の型は、エラーとなる
        //理由) サイズが不明なため
        // enum List {
        //     Cons(i32, List),
        //     Nil,
        // }
        //**Boxにすることで、一つ一つのサイズが確定する**//
        //理由) そのデータとポインタデータの2つのみを持つため
        //スタックに格納されるデータをヒープに格納することができる。その時スタックに残るのはヒープへのポインタ。
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        //useキーワードでは、複数のenumの型をこのように呼び出せる
        use List::{Cons, Nil};

        //問題なく利用できている
        let list: Box<List> = Box::new(List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        ));
    }

    {
        //***Derefトレイト***//
        //*参照外しが使えるようになるトレイト
        mod tests {
            #[cfg(test)]
            #[test]
            fn deref_test() {
                //*Boxには、実装されている
                let x = 5;
                let borrow_x = Box::new(x);

                //ちゃんと参照外ししないとエラーになる
                //つまり、Derefトレイトがあるということ (参照外しができるため)
                println!("Derefトレイト");
                assert_eq!(5, *borrow_x);
            }

            #[test]
            fn my_deref_test() {
                //*独自のDerefトレイトを実装する
                use std::ops::Deref;

                struct MyBox<T>(T);

                impl<T> MyBox<T> {
                    fn new(x: T) -> Self {
                        Self(x)
                    }
                }

                //これを実装することで、初めて参照外しができるようになる
                //パターン (覚えなくてもよい)
                //T: Deref<Target=U> ... &T から &U
                //T: DerefMut<Target=U> ... &mut T から &mut U
                //T: FDeref<Target=U> ... &mut T から &U
                impl<T> Deref for MyBox<T> {
                    //参照外しを行った際に得られる型を設定する。
                    //中身の型の参照を返す
                    type Target = T;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                //実際に使用する
                let x: i32 = 5;
                let y: MyBox<i32> = MyBox::new(5);

                println!("独自のDerefトレイト");
                assert_eq!(5, x);
                assert_eq!(5, *y); //実際は、*(y.deref())となっていた

                //Stringから&strに変換できる
                fn hello(name: &str) {
                    println!("Hello, {}!", name);
                }

                //* &str ... 文字列スライス &[u8]
                //* String ... ベクタ型のラッパ Vec<u8, Global>
                //MyBox<T>にDerefトレイトを実装したので、コンパイラはderefを呼び出すことで、
                //&MyBox<String>を&Stringに変換できるのです。標準ライブラリは、
                //Stringに文字列スライスを返すDerefの実装を提供していて、 この実装は、DerefのAPIドキュメンテーションに載っています。
                //コンパイラはさらにderefを呼び出して、 &Stringを&strに変換し、これはhello関数の定義と合致します。
                let m = MyBox::new(String::from("Rust"));
                hello(&m);

                //実際は、以下のようになる
                //String -> &MyBox<String> -> &String (MyBoxの中身の参照) = m_true.defer() -> String = *(m_true.defer()) -> str = *(String.defer()) = (*m_true)[..] -> &str
                let m_true = MyBox::new(String::from("Rust True"));
                hello(&(*m_true)[..]);
            }
        }
    }

    {
        //***Dropトレイト***//
        //*スコープから抜けるときに、追加の処理ができるトレイト
        {
            struct CustomSmartPointer {
                data: String,
            }

            //独自のDropトレイトを実装する
            impl Drop for CustomSmartPointer {
                fn drop(&mut self) {
                    println!("Dropping CustomSmartPointer with data {}", self.data);
                }
            }

            //対象のスコープ
            //Dropする順番は、逆になる
            println!("\n");
            println!("Dropトレイト");
            {
                let _a: CustomSmartPointer = CustomSmartPointer {
                    data: String::from("variable a"),
                };
                let _b: CustomSmartPointer = CustomSmartPointer {
                    data: String::from("variable b"),
                };
                println!("Inside the Scope");
            }

            //*Dropトレイトを呼ぶ方法 *//
            //*直接は呼べない *//
            //* std::mem::drop(struct)関数でドロップさせることができる *//
            {
                println!("std::mem::drop(struct)を使用した例");
                use std::mem::drop; //要らないらしい

                let _a: CustomSmartPointer = CustomSmartPointer {
                    data: String::from("variable a"),
                };

                //ここでドロップする
                drop(_a);

                let _b: CustomSmartPointer = CustomSmartPointer {
                    data: String::from("variable b"),
                };
                let _c: CustomSmartPointer = CustomSmartPointer {
                    data: String::from("variable c"),
                };
                println!("Inside the Scope");
            }
        }
    }

    {
        //***Rcトレイト***//
        //*同じ値に対して複数の所有者がいることを許す*//
        //*所有者の数の情報を保持している*//
        //*所有者が0になった時に、メモリを開放する*//
        //*シングルスレッドのときだけ使える*//
        //?読み取り専用であることに注意！ *//
        {
            {
                //Rcを使わないとできない
                enum ListBefore {
                    Cons(i32, Box<ListBefore>),
                    Nil,
                }

                use ListBefore::{Cons, Nil};

                let a: Box<ListBefore> =
                    Box::new(Cons(5, Box::new(Cons(10, Box::new(ListBefore::Nil)))));
                let b: ListBefore = Cons(3, Box::new(*a)); /*aは、bにmoveされる*/
                //let c: List = Cons(4, Box::new(*a)); aは、bにmoveされたためにcへは、moveできない
            }

            {
                //Rcを使うと、bとcにaへの参照を持てる
                enum List {
                    Cons(i32, Rc<List>),
                    Nil,
                }

                use std::rc::Rc; //要らないらしい
                use List::{Cons, Nil};

                //*Rc::new()で、生成する
                let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(List::Nil)))));
                //*Rc::clone(&struct)で、所有権をその変数に渡す (このとき、moveされない)
                let _b: List = Cons(3, Rc::clone(&a));
                let _c: List = Cons(4, Rc::clone(&a));
                //データは、こんな感じ
                // a:      5 -> 10 -> Nil;
                //_b: 3 -> 5 -> 10 -> Nil;
                //_c: 4 -> 5 -> 10 -> Nil;

                {
                    //*所有権を持つ変数の数が変化する様子*//
                    println!("所有権を持つ変数の数が変化する様子");
                    //Rc::strong_count(&元のstruct <- Rc::new()で作ったやつ))で、所有者の数を確認できる
                    let a: Rc<List> =
                        Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
                    println!("count after creating a = {}", Rc::strong_count(&a));
                    let _b: List = List::Cons(3, Rc::clone(&a));
                    println!("count after creating _b = {}", Rc::strong_count(&a));
                    {
                        let _c: List = List::Cons(4, Rc::clone(&a));
                        println!("count after creating _c = {}", Rc::strong_count(&a));
                    }
                    //_cがスコープから抜けた後のカウント
                    println!(
                        "count after _c goes out of scope = {}",
                        Rc::strong_count(&a)
                    );
                }
            }

            {
                //実験
                //*前提) Rustでは、借用中の変数を変更することができないが、再宣言を行うことは可能
                //*前提) 1つの可変参照かいくつもの不変参照のどちらかが可能になる
                //*回答: 不変参照をしているために、シャドーイングを行うことで参照先の更新したデータに変更できる*//
                println!("実験");
                println!("Rcの謎に迫る");
                use std::rc::Rc;

                #[derive(Debug)]
                struct Foo {
                    value: i32,
                    foo_rc: Option<Rc<Foo>>,
                }

                let mut foo1: Rc<Foo> = Rc::new(Foo {
                    value: 1,
                    foo_rc: None,
                });
                let foo2: Foo = Foo {
                    value: 2,
                    foo_rc: Some(Rc::clone(&foo1)),
                };
                let foo3: Foo = Foo {
                    value: 3,
                    foo_rc: Some(Rc::clone(&foo1)),
                };

                println!("foo1: {:?}", foo1);
                println!("foo2: {:?}", foo2);
                println!("foo3: {:?}", foo3);

                foo1 = Rc::new(Foo {
                    value: 1,
                    foo_rc: Some(Rc::new(Foo {
                        value: 0,
                        foo_rc: None,
                    })),
                });

                let foo2: Foo = Foo {
                    value: 2,
                    foo_rc: Some(Rc::clone(&foo1)),
                };
                let foo3: Foo = Foo {
                    value: 3,
                    foo_rc: Some(Rc::clone(&foo1)),
                };

                println!("foo1を変化させた後");
                println!("foo1: {:?}", foo1);
                println!("foo2: {:?}", foo2);
                println!("foo3: {:?}", foo3);
            }
            {
                //***RefCellトレイト***//
                //*不変参照を可変参照に変更する*//
                //*シングルスレッドのときだけ使える*//
                //*実行時に精査される (その分遅くなり、安全性の担保も薄くなる <-　循環参照の可能性) */
                use std::cell::RefCell;
                {
                    //以下のコードは、エラーになる
                    //* let x = 5; //不変な変数
                    //* let y = &mut x; //可変な参照
                    //不変な変数に可変な参照は、取れない
                    //*しかし、以下のコードは正しく実行できる
                    println!("\n");
                    println!("RefCellトレイト");
                    let x: RefCell<i32> = RefCell::new(5); //これは、どういう状態
                    let y = x.borrow_mut();
                    println!("y: {}", y);
                };
            }
        }
    }
}
