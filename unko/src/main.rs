fn main() {
    let a: i32 = 10;
    let sample = |b: &&i32| {**b};

    println!("{}", sample(&&a));
}