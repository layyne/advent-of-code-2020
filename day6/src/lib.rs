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

fn count_yes(filename: &String) -> Result<u32, Box<dyn Error>> {
    let lines = read_lines(filename)?;
    
    let mut questions_int: u32 = 0;
    let mut total = 0;

    for line in lines {
        let cur = line?;

        if cur == "" {
            total += questions_int.count_ones();

            questions_int = 0;

            continue;
        }

        for c in cur.chars() {
            let index = c as u8 - 'a' as u8;
            questions_int |= 1 << index;
        }
    }

    total += questions_int.count_ones();

    Ok(total as u32)
}

fn count_unanimous(filename: &String) -> Result<u32, Box<dyn Error>> {
    let lines = read_lines(filename)?;

    let mut questions: [u32; 26] = [0; 26];
    let mut total = 0;
    let mut ppl = 0;

    for line in lines {
        let cur = line?;

        if cur == "" {
            total += questions.iter().filter(|&&q| q == ppl).count();

            ppl = 0;
            questions = [0; 26];

            continue;
        }

        for c in cur.chars() {
            let index = c as u8 - 'a' as u8;
            questions[index as usize] += 1;
        }

        ppl += 1;
    }

    total += questions.iter().filter(|&q| *q == ppl).count();

    Ok(total as u32)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Sum of p1 counts is {}", count_yes(&config.filename)?);

    println!("Sum of p2 counts is {}", count_unanimous(&config.filename)?);

    Ok(())
}
