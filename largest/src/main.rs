fn search_largest<T: PartialOrd>(number_vec: &[T]) -> &T {
    let mut largest: &T = &number_vec[0];
    for num in number_vec {
        if num > largest {
            largest = num
        }
    }
    largest
}

fn main() {
    let number_vec: Vec<i32> = vec![34, 50, 25, 100];
    let number_vec2: Vec<i32> = vec![34, 5000, 25, 100];

    println!("The lagest number is {}", search_largest(&number_vec));
    println!("The lagest number is {}", search_largest(&number_vec2));

    //参照外し
    let _a: i32 = *search_largest(&number_vec);
    //これで、参照を外せる (Copy Traitを実装されている型のみ)
    //Copy Traitを実装していない型は、clone()メソッドを用いる (Clone Traitを実装している型のみ)
}