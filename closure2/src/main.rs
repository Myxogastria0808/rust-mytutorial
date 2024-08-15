use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: PartialOrd + Eq + Hash + Copy,
    V: Clone,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: PartialOrd + Eq + Hash + Copy,
    V: Clone,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::<U, V>::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        //argの取得を試みる
        let value: Option<&V> = self.value.get(&arg);
        match value {
            //cloneすることによって、スコープを抜けても戻り値として返せる
            Some(v) => (*v).clone(),
            None => {
                let v: V = (self.calculation)(arg);
                self.value.insert(arg, v.clone());
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //closureの型推論が行われている
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value: u32 = 10;
    let simulated_random_number: u32 = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let power = |a: i32| a.pow(8u32);

    let mut c1 = Cacher::new(power);
    let c1_result1 = c1.value(2);
    println!("c1_result1: {c1_result1}");
}
