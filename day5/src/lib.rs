use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;
use std::error::Error;

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

    for line in lines {
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

        let row = range.0 as u32;

        range = (0, 7);

        for c in split.1.chars() {
            range = match c {
                'L' => (range.0, (range.0 + range.1) / 2),
                'R' => ((range.0 + range.1) / 2 + 1, range.1),
                _ => return Err("shit".into()),
            }
        }

        let col = range.0 as u32;

        let seat_id = row * 8 + col;

        if seat_id > max {
            max = seat_id;
        }
    }

    Ok(max)
}

fn find_missing(filename: &String) -> Result<i32, Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let mut max: i32 = 0;
    let mut min: i32 = 127 * 8 + 7;
    let mut vec: Vec<i32> = Vec::new();

    for line in lines {
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

        let row = range.0 as i32;

        range = (0, 7);

        for c in split.1.chars() {
            range = match c {
                'L' => (range.0, (range.0 + range.1) / 2),
                'R' => ((range.0 + range.1) / 2 + 1, range.1),
                _ => return Err("shit".into()),
            }
        }

        let col = range.0 as i32;

        let seat_id = row * 8 + col;

        if seat_id > max {
            max = seat_id;
        } else if seat_id < min {
            min = seat_id;
        }

        vec.push(seat_id);
    }

    // Avoid overflow, sum all numbers in min+1..max+1 and find which num is missing

    let mut sum = min;

    for i in min..max {
        sum += i + 1;
        sum -= vec.get((i - min) as usize).expect("out of vec bounds");
    }

    Ok(sum)
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

    println!(
        "The missing seat ID is {}",
        find_missing(&config.filename)?,
    );

    Ok(())
}
