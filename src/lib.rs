use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn count_valid_passwords(filename: &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut count = 0;
        let mut valid_count = 0;
        let mut buffer = String::new();

        for line in lines {
            let line = line.expect("line fail");

            let mut chars = line.chars();

            let mut c = chars.next().expect("char fail");

            while c.is_digit(10) {
                buffer.push(c);
                c = chars.next().expect("char fail");
            }

            let min = buffer.parse::<i32>().unwrap();

            buffer.clear();

            c = chars.next().expect("char fail");

            while c.is_digit(10) {
                buffer.push(c);
                c = chars.next().expect("char fail");
            }

            let max = buffer.parse::<i32>().unwrap();

            buffer.clear();

            let req = chars.next().expect("char fail");

            chars.next();
            chars.next();

            let mut c = chars.next();

            while c != None {
                if c.expect("char fail") == req {
                    count += 1;
                }

                c = chars.next();
            }

            if count >= min && count <= max {
                valid_count += 1;
            }

            count = 0;
        }

        valid_count
    } else {
        println!("failed to read file");
        -1
    }
}

pub fn count_valid_passwords_2_electric_boogaloo(filename: &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut valid_count = 0;
        let mut buffer = String::new();

        for line in lines {
            let line = line.expect("line fail");

            let mut chars = line.chars();

            let mut c = chars.next().expect("char fail");

            while c.is_digit(10) {
                buffer.push(c);
                c = chars.next().expect("char fail");
            }

            let index1 = buffer.parse::<i32>().unwrap();

            buffer.clear();

            c = chars.next().expect("char fail");

            while c.is_digit(10) {
                buffer.push(c);
                c = chars.next().expect("char fail");
            }

            let index2 = buffer.parse::<i32>().unwrap();

            buffer.clear();

            let req = chars.next().expect("char fail");

            chars.next();
            chars.next();

            if (chars.nth((index1 - 1) as usize).unwrap() == req)
                ^ (chars.nth((index2 - index1 - 1) as usize).unwrap() == req)
            {
                valid_count += 1;
            }
        }

        valid_count
    } else {
        println!("failed to read file");
        -1
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
