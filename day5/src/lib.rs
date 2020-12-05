use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() != 2 {
            return Err("Usage: cargo run <filename>".into());
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

fn find_max_seat_id(filename: &String) -> Result<u32, Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let mut max: u32 = 0;
    let mut row: u32;
    let mut col: u32;
    let mut seat_id: u32;

    for (i, line) in lines.enumerate() {
        let mut range: (u8, u8) = (0, 127);

        let cur = line?;

        let split = cur.split_at(7);

        for c in split.0.chars() {
            range = match c {
                'F' => (range.0, range.0 + (range.1 - range.0) / 2),
                'B' => (range.0 + (range.1 - range.0) / 2 + 1, range.1),
                _ => return Err("fuck".into()),
            }
        }

        row = range.0 as u32;

        print!("line {}:    row = {}    ", i + 1, row);
        
        range = (0, 7);
        
        for c in split.1.chars() {
            range = match c {
                'L' => (range.0, (range.0 + range.1) / 2),
                'R' => ((range.0 + range.1) / 2 + 1, range.1),
                _ => return Err("shit".into()),
            }
        }
        
        col = range.0 as u32;
        
        print!("col = {}    ", col);

        seat_id = row * 8 + col;

        if seat_id > max {
            print!("new max: {}", seat_id);
            max = seat_id;
        }

        println!("");
    }

    Ok(max)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!(
        "The highest seat ID is {}",
        find_max_seat_id(&config.filename)?,
    );

    Ok(())
}
