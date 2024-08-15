//***Trait***//
//メソッドシグネチャのこと
pub trait Summary {
    //デフォルト実装
    //デフォルト実装は、オーバーライド可能
    fn summarize(&self) -> String {
        String::from("(Read more ...)")
    }
}

pub trait SummaryTweet {
    //このようにtraitを定義した場合は、summarize_author関数をimpleメソッドで用いればよい
    fn summarize_author(&self) -> String;
    fn summarize_by(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//デフォルト実装そのままを使うときは、以下のようにする
impl Summary for NewsArticle {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    //デフォルト実装のオーバーライド
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl SummaryTweet for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("HeadLine!!!!!!!!!"),
        location: String::from("Earth"),
        author: String::from("Human"),
        content: String::from("My name is Yuki Osada!"),
    };

    println!("News article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("にょき"),
        content: String::from("yheeeeeeee!!!!!!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize_by());
}
