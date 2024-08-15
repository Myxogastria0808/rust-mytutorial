use std::env;

use minigrep::Config;

fn main() {
    let args: std::env::Args = env::args();
    println!("{:?}", &args);

    let config: Config = Config::new(args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1)
    });

    println!("Serching for: {}", config.query);
    println!("In file: {}", config.filename);
    println!("\n");

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    };
}
