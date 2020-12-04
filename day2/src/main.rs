use std::env;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: cargo run <filename>");
    }

    println!(
        "The number of valid passwords for part 1 is {}, for part 2 is {}",
        lib::count_valid_passwords(&args[1]),
        lib::count_valid_passwords_2_electric_boogaloo(&args[1]),
    );
}
