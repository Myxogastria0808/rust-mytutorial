fn main() {
    {
        let v1: Vec<i32> = vec![1, 2, 3];
        //イテレータの生成
        //.iter() ... 不変な参照をしている
        //.iter_into() ... moveする
        // .iter_mut() ... 可変な参照をしている
        let v1_iter: std::slice::Iter<'_, i32> = v1.iter();

        //イテレータは、このようにforループで要素を取り出せる。
        for val in v1_iter {
            println!("Get: {}", val)
        }
    }
    //***イテレータアダプタ***//
    //参考
    //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
    {
        //***sum()メソッド***//
        //要素を消費するため、v1_iterはもう使えない
        let v1: Vec<i32> = vec![1, 2, 3];
        let v1_iter: std::slice::Iter<'_, i32> = v1.iter();

        let total: i32 = v1_iter.sum();
        println!("total: {total}");
    }
    {
        //***map()メソッド***//
        //map(クロージャ) の記述でiterator内の要素を変化させる
        //iteratorを消費していないので、Rustに怒られる <-自動的に消費されないため
        let v1: Vec<i32> = vec![1, 2, 3];

        //消費すると、Rustはwarningを消す
        let v1_result = v1.iter().map(|x: &i32| -> i32 { x + 1 });
        for val in v1_result {
            println!("Get: {}", val)
        }

        //*消費したいならば、collect()メソッドを使うのが良い*//
        //*collect()を使う方がよさそう */
        //*↑各要素に変更を加えるメソッドチェインの時は、良い */
        let v2: Vec<i32> = vec![1, 2, 3];
        let v2_result: Vec<_> = v2.iter().map(|x: &i32| -> i32 { x + 1 }).collect();
        println!("v2: result: {:?}", v2_result);
    }
    {
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        let shoes: Vec<Shoe> = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        println!("Shoe: {:?}", shoes);

        fn filters_by_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes
                .into_iter()
                .filter(|s: &Shoe| s.size == shoe_size)
                .collect()
        }

        let shoes_after: Vec<Shoe> = filters_by_size(shoes, 10);

        println!("Shoe: {:?}", shoes_after);
    }
    {
        //***これがいるらしい***//
        use std::iter::Iterator;

        //*自作のIterator Trait*//
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Self {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;

                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let counter: Counter = Counter::new();
        for item in counter {
            println!("{}", item);
        }

        //*自作のイテレータでもメソッドが自由に使える！！！
        fn using_other_iterator_trait_methods() -> u32 {
            let sum: u32 = Counter::new()
                .zip(Counter::new().skip(1))
                .map(|(a, b)| a * b)
                .filter(|x| x % 3 == 0)
                .sum();
            sum
        }
        println!(
            "using_other_iterator_trait_methods: {}",
            using_other_iterator_trait_methods()
        )
    }
}

//Iteratorトレイト
//Iteratorトレイトは、以下のように標準実装されている
pub trait Iterator {
    type Item; //<-nectメソッドの戻り値の型に使われる
    fn next(&mut self) -> Option<Self::Item>; //<-繰り返しが終わったら、Noneを
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let mut v1_iter: std::slice::Iter<'_, i32> = v1.iter();

        //nextの各呼び出しは、イテレータの要素を一つ、食います。
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
