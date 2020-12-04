use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;

struct Map {
    filename: String,
    width: i32,
}

pub struct Config {
    map: Map,
    h_slope: i32,
    v_slope: i32,
}

impl Map {
    fn new(filename: String) -> Result<Map, Box<dyn Error>> {
        let width = read_lines(filename.clone())?.next().unwrap()?.len() as i32;

        Ok(Map { filename, width })
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() != 4 {
            return Err("Usage: cargo run <filename> <h_slope> <v_slope>".into());
        }

        let filename = args[1].clone();
        let h_slope = args[2].clone().parse::<i32>()?;
        let v_slope = args[3].clone().parse::<i32>()?;

        let map = Map::new(filename)?;

        Ok(Config {
            map,
            h_slope,
            v_slope,
        })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn traverse_map(map: Map, h_slope: i32, v_slope: i32) -> Result<i32, Box<dyn Error>> {
    let lines = read_lines(map.filename)?;
    let mut line_count = -1;
    let mut count: i32 = 0;
    let mut col: i32 = 0;

    for line in lines {
        line_count += 1;

        if line_count % v_slope != 0 {
            continue;
        }

        let line = line?;

        if line.chars().nth(col as usize).unwrap() == '#' {
            count += 1;
        }

        col = (col + h_slope) % map.width;
    }

    Ok(count)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "{} trees encountered with the supplied map/slope",
        traverse_map(config.map, config.h_slope, config.v_slope)?
    );

    Ok(())
}
