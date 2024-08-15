use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    {
        //リファクタリング前
        //重たい計算をする関数が何度も呼び出されてしまっている

        //非常に重たい計算
        fn simulated_expensive_calculation(intensity: u32) ->  u32 {
            println!("calculating slowly ...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

        fn generate_workout(intensity: u32, random_number: u32) {
            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    simulated_expensive_calculation(intensity)
                );
                println!(
                    "Next, do {} situps!",
                    simulated_expensive_calculation(intensity)
                );
            } else {
                if random_number == 3 {
                    println!(
                        "Take a break today! Remember to stay hydrated!"
                    );
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        simulated_expensive_calculation(intensity)
                    )
                }
            }
        }

        //実行する処理
        let simulated_user_specified_value: u32 = 10;
        let simulated_random_number: u32 = 7;

        generate_workout(
            simulated_user_specified_value,
            simulated_random_number
        );
        println!("====================================");
    }

    {
        //リファクタリング パターン1
        //ちょっと良くなって、if文では、処理を2回しなくてよくなった
        //しかし、intensity => 25 && random_number == 3 の時は、重い計算をしなくてもいいのに計算してしまう。

        //非常に重たい計算はそのまま
        fn simulated_expensive_calculation_ref1(intensity: u32) ->  u32 {
            println!("calculating slowly ...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

        fn generate_workout_ref1(intensity: u32, random_number: u32) {
            let expensive_result: u32 = simulated_expensive_calculation_ref1(intensity);
            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    expensive_result
                );
                println!(
                    "Next, do {} situps!",
                    expensive_result
                );
            } else {
                if random_number == 3 {
                    println!(
                        "Take a break today! Remember to stay hydrated!"
                    );
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_result
                    )
                }
            }
        }

        //実行する処理
        let simulated_user_specified_value: u32 = 10;
        let simulated_random_number: u32 = 7;

        generate_workout_ref1(
            simulated_user_specified_value,
            simulated_random_number
        );
        println!("====================================");
    }

    {
        //***クロージャでリファクタリング (パターン2)***//
        //***実は、これは意味がない (パターン1と実質同じ)***//
        fn generate_workout_closure(intensity: u32, random_number: u32) {
            //*非常に重たい計算をクロージャにする***/
            let simulated_expensive_calculation_closure = |intensity: u32| ->  u32 {
                println!("calculating slowly ...");
                thread::sleep(Duration::from_secs(2));
                intensity
            };
            //*クロージャは、型推論をするので、型を省略できる。(俺は、しないかも) *//
            //*1回、あるクロージャを呼び出したら、その後、そのクロージャは別の型では使えない。*//
            //*参考までに*//
            // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
            // let add_one_v2 = |x: u32| -> u32 { x + 1 };
            // let add_one_v3 = |x|             { x + 1 };
            // let add_one_v4 = |x|               x + 1  ;


            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    simulated_expensive_calculation_closure(intensity)
                );
                println!(
                    "Next, do {} situps!",
                    simulated_expensive_calculation_closure(intensity)
                );
            } else {
                if random_number == 3 {
                    println!(
                        "Take a break today! Remember to stay hydrated!"
                    );
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        simulated_expensive_calculation_closure(intensity)
                    )
                }
            }
        }

        //実行する処理
        let simulated_user_specified_value: u32 = 10;
        let simulated_random_number: u32 = 7;

        generate_workout_closure(
            simulated_user_specified_value,
            simulated_random_number
        );
        println!("====================================");
    }

    {
        //***以下のように、構造体を作成しておくパターン***//
        //***実は、以下の構造体にも問題がある */
        //*問題点1 ... 実質ジェネリックな型でない *//
        //*問題点2 ... valueに値の再代入ができない (値があるかどうかの判定をしているため) *//
        //***Closureの型の種類について***//
        //3種類存在//
        //*基本は、Fn(T) -> T でOK*//
        //*コンパイルエラーが出たら、考える*//
        //FnOnce ... クロージャの環境として知られている内包されたスコープからキャプチャした変数を消費します。 
        //キャプチャした変数を消費するために、定義された際にクロージャはこれらの変数の所有権を奪い、 自身にムーブするのです。
        //名前のうち、Onceの部分は、 このクロージャは同じ変数の所有権を2回以上奪うことができないという事実を表しているので、
        //1回しか呼ぶことができないのです。
        //FnMut ... 可変で値を借用するので、環境を変更することができます。
        //Fn ... 環境から値を不変で借用します。

        struct Cacher<T, U, V>
            where T: Fn(U) -> V,
                    U: Copy,
                    V: Copy,
        {
            _calculation: T, //<-これがトレイト
            value: HashMap<String, U>,
        }

        impl<T, U, V> Cacher<T, U, V>
            where T: Fn(U) -> V,
                    U: Copy,
                    V: Copy,
        {
            fn new(_calculation: T, value: HashMap<String, U>) -> Self {
                Cacher {
                    _calculation,
                    value,
                }
            }

            fn value<'a>(&mut self, arg: &'a U) -> &'a V {
                let value: Option<&U> = (self.value).get(&String::from("value"));
                if let Some(stored_value) = value {
                    if stored_value != arg {
                        (self.value).insert(String::from("value_fnonce"), *arg);
                    }
                }
                arg
            }
        }

        fn generate_workout_closure_i32(intensity: i32, random_number: u32) {
            //***_calculationのclosureを定義する***//
            let mut init_hashmap: HashMap<String, i32> = HashMap::new();
            init_hashmap.insert(String::from("value"), 0);
            let mut expensive_result = Cacher::new(|num| {
                println!("calculationg slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            }, init_hashmap);

            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    expensive_result.value(&intensity)
                );
                println!(
                    "Next, do {} situps!",
                    expensive_result.value(&intensity)
                );
            } else {
                if random_number == 3 {
                    println!(
                        "Take a break today! Remember to stay hydrated!"
                    );
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_result.value(&intensity)
                    )
                }
            }
        }

        fn generate_workout_closure_f64(intensity: f64, random_number: u32) {
            //***calculationのclosureを定義する***//
            let mut init_hashmap: HashMap<String, f64> = HashMap::new();
            init_hashmap.insert(String::from("value"), 0.00);
            let mut expensive_result = Cacher::new(|num| {
                println!("calculationg slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            }, init_hashmap);

            if intensity < 25.0 {
                println!(
                    "Today, do {} pushups!",
                    expensive_result.value(&intensity)
                );
                println!(
                    "Next, do {} situps!",
                    expensive_result.value(&intensity)
                );
            } else {
                if random_number == 3 {
                    println!(
                        "Take a break today! Remember to stay hydrated!"
                    );
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_result.value(&intensity)
                    )
                }
            }
        }

        //クロージャが型推論を行うために、以下のように型が異なる
        //実行する処理
        let simulated_user_specified_value: i32 = 10;
        let simulated_random_number: u32 = 7;

        generate_workout_closure_i32(
            simulated_user_specified_value,
            simulated_random_number
        );
        println!("====================================");
        //実行する処理
        let simulated_user_specified_value: f64 = 10.000;
        let simulated_random_number: u32 = 7;

        generate_workout_closure_f64(
            simulated_user_specified_value,
            simulated_random_number
        );
        println!("====================================");
    }
}

//***クロージャで環境をキャプチャする***//
//*以下のコードは、正しく動く!*//
//*closure内の変数でもないxが正しく評価？されている*//
#[cfg(test)]
mod tests {
    #[test]
    fn capch() {
        let x: i32 = 4;
        let equal_to_x = |z: i32| z == x;
        let y: i32 = 4;
        assert!(equal_to_x(y));
    }

    #[test]
    fn capch2() {
        let x: Vec<i32> = vec![1,2,3];
        //moveを付けると、xから所有権が移動する (ヒープ領域に保存される場合のものについては)
        let equal_to_x = move |z| z == x;
        let y: Vec<i32> = vec![1,2,3];
        equal_to_x(y);
        //以下は、エラーとなる
        //assert_eq!(vec![1,2,3], x);
    }
}