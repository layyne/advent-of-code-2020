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
            if args.len() != 4 {
                return Err("Usage: cargo run <filename>".into());
            }
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

fn count_valid_pp_1(filename: String) -> Result<u32, Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let mut field_count = 0;
    let mut valid_count = 0;

    for line in lines {
        let cur = line?;

        if cur.len() == 0 {
            if field_count == 7 {
                valid_count += 1;
            }

            field_count = 0;
            continue;
        }

        let cur = cur.trim().split_whitespace();

        for field in cur {
            let field = field.split(':').collect::<Vec<&str>>();

            field_count += match field[0] {
                "byr" => 1,
                "iyr" => 1,
                "eyr" => 1,
                "hgt" => 1,
                "hcl" => 1,
                "ecl" => 1,
                "pid" => 1,
                _ => 0,
            };
        }
    }

    if field_count == 7 {
        valid_count += 1;
    }

    Ok(valid_count)
}

fn count_valid_pp_2(filename: String) -> Result<u32, Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let mut field_count = 0;
    let mut valid_count = 0;

    for line in lines {
        let cur = line?;

        if cur.len() == 0 {
            if field_count == 7 {
                valid_count += 1;
            }

            field_count = 0;
            continue;
        }

        let cur = cur.trim().split_whitespace();

        for field in cur {
            let field = field.split(':').collect::<Vec<&str>>();

            field_count += match field[0] {
                "byr" => {
                    let data = field[1].parse::<u32>()?;

                    if data >= 1920 && data <= 2002 {
                        1
                    } else {
                        0
                    }
                }
                "iyr" => {
                    let data = field[1].parse::<u32>()?;

                    if data >= 2010 && data <= 2020 {
                        1
                    } else {
                        0
                    }
                }
                "eyr" => {
                    let data = field[1].parse::<u32>()?;

                    if data >= 2020 && data <= 2030 {
                        1
                    } else {
                        0
                    }
                }
                "hgt" => {
                    let mut unit_index = 0;

                    for c in field[1].chars() {
                        if c.is_alphabetic() {
                            break;
                        }

                        unit_index += 1;
                    }

                    let split = field[1].split_at(unit_index);
                    let data = split.0.parse::<u32>()?;

                    match split.1 {
                        "cm" => {
                            if data >= 150 && data <= 193 {
                                1
                            } else {
                                0
                            }
                        }
                        "in" => {
                            if data >= 59 && data <= 76 {
                                1
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    }
                }
                "hcl" => {
                    if field[1].chars().nth(0).unwrap() != '#' {
                        0
                    } else if field[1][1..].len() != 6 {
                        0
                    } else {
                        let mut valid = 1;

                        for c in field[1][1..].chars() {
                            if !c.is_digit(16) {
                                valid = 0;
                                break;
                            }
                        }

                        valid
                    }
                }
                "ecl" => match field[1] {
                    "amb" => 1,
                    "blu" => 1,
                    "brn" => 1,
                    "gry" => 1,
                    "grn" => 1,
                    "hzl" => 1,
                    "oth" => 1,
                    _ => 0,
                },
                "pid" => match field[1].len() {
                    9 => 1,
                    _ => 0,
                },
                _ => 0,
            };
        }
    }

    if field_count == 7 {
        valid_count += 1;
    }

    Ok(valid_count)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "The number of valid passports for part 1 is {}",
        count_valid_pp_1(config.filename.clone())?,
    );

    println!(
        "The number of valid passports for part 2 is {}",
        count_valid_pp_2(config.filename)?,
    );

    Ok(())
}
