use day3::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Argument parse error: {}", err);
        process::exit(1);
    });

    if let Err(e) = day3::run(config) {
        println!("run() error: {}", e);
        process::exit(1);
    };
}
