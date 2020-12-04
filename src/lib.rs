use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn find_triplet_with_sum(filename: &str, sum: i32) -> (i32, i32, i32) {
    if let Ok(lines) = read_lines(filename) {
        let mut summands = (i32::MAX, i32::MAX, i32::MAX);

        for line in lines {
            if let Ok(entry) = line {
                let entry = entry.parse::<i32>().unwrap();

                let pair = find_pair_with_sum(filename, sum - entry);

                if pair != (i32::MAX, i32::MAX) {
                    summands = (pair.0, pair.1, entry);
                    break;
                }
            }
        }

        summands
    } else {
        println!("failed to read file");
        (i32::MAX, i32::MAX, i32::MAX)
    }
}

pub fn find_pair_with_sum(filename: &str, sum: i32) -> (i32, i32) {
    if let Ok(lines) = read_lines(filename) {
        let mut summands = (i32::MAX, i32::MAX);

        let mut compls: HashSet<i32> = HashSet::new();

        for line in lines {
            if let Ok(entry) = line {
                let entry = entry.parse::<i32>().unwrap();

                if compls.contains(&entry) {
                    summands = (entry, sum - entry);
                    break;
                }

                compls.insert(sum - entry);
            }
        }

        summands
    } else {
        println!("failed to read file");
        (i32::MAX, i32::MAX)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
