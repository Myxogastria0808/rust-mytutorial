pub mod generic;
use crate::generic::*;

fn main() {
    //struct Point1<i32>
    let point11: Point1<i32> = Point1::set_point(10, 20);
    let get_point11: (&i32, &i32) = point11.get_point();
    println!("Point1 -> x: {}, y: {}", get_point11.0, get_point11.1);
    //struct Point1<f64>
    let point12: Point1<f64> = Point1::set_point(10.1, 20.2);
    let get_point12: (&f64, &f64) = point12.get_point();
    println!("Point -> x: {}, y: {}", get_point12.0, get_point12.1);

    //struct Point2<f64, i32>
    let point21: Point2<f64, i32> = Point2::set_point_x_f64(10.0001, -1);
    let get_point21: (&f64, &i32) = point21.get_point_x_f64();
    println!("Point -> x: {}, y {}", get_point21.0, get_point21.1);
    //以下は、エラーとなる
    //let point22: Point2<f32, i32> = Point2::set_point_x_f64(10.0001, -1);

    //enum User<T, U>
    let user1: User<String> = User::Username(String::from("Hello, World!"));
    let user2: User<&str> = User::Username("Hello, World!");
    match user1 {
        User::Username(user1) => println!("user1: {}", user1),
        User::Nickname(user1) => println!("user1: {}", user1),
    }
    match user2 {
        User::Username(user2) => println!("user2: {}", user2),
        User::Nickname(user2) => println!("user2: {}", user2),
    }
}