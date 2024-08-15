use std::fmt::{Display, Debug};

//Trait
//impl Trait構文 (省略記法的なもの)
trait SayNameImpl {
    fn say_name(name: &impl Display) -> () {
        println!("{}", name);
    }
}

//Trait境界構文 (省略記法)
trait SayNameBoaderShort {
    fn say_name<T: Display>(name: &T) -> () {
        println!("{}", name);
    }
}

//Trait境界構文 (本来の書き方)
trait SayNameBoader {
    fn say_name<T>(name: &T) -> ()
        where T: Display
    {
        println!("{}", name);
    }
}

//インスタンスそのものを返すときは、&selfのみでOK!
trait ShowSample {
    fn show_sample(&self) -> ()
        where Self: Debug
    {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct Sample {
    username: String,
    nickname: String,
    age: u8,
}

impl SayNameImpl for Sample {}
impl ShowSample for Sample {}

fn main() {
    let unko: Sample = Sample {
        username: "unko".to_string(),
        nickname: "unko chan".to_string(),
        age: 60,
    };
    Sample::show_sample(&unko);
    Sample::say_name(&unko.age);
}
