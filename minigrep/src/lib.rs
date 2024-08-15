use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f: File = File::open(config.filename)?;

    let mut contents: String = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

//これは、関連関数
//関連関数の時の呼び出しは、構造体名::関連関数名 でできる！
//メソッドであれば、インスタンス名.メソッド名 でできる！
//'static <-常に有効な参照
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        //最初の一つ目の要素は、必要ないので飛ばす
        args.next();
        let query: String = match args.next() {
            Some(query) => query,
            None => return Err("Didn`t get a query atring."),
        };
        let filename: String = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn`t get a filename string."),
        };
        //新しいstructを作るときは、参照ではなく実態である必要がある。
        Ok(Self { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
