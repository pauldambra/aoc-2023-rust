mod day_one;
mod day_two;
mod day_three;
mod r#aoc;

use day_one::day_one;
use day_two::day_two_part_one;
use day_two::day_two_part_two;
use day_three::day_three_part_one;
use std::io::{BufReader, Error};
use std::fs::File;
use std::env;
use crate::day_three::day_three_part_two;

fn main() {
    if let Err(err) = day_one() {
        eprintln!("Error: {}", err);
    }
    if let Err(err) = day_two_part_one() {
        eprintln!("Error: {}", err);
    }
    if let Err(err) = day_two_part_two() {
        eprintln!("Error: {}", err);
    }
    if let Err(err) = day_three_part_one() {
        eprintln!("Error: {}", err)
    }
    if let Err(err) = day_three_part_two() {
        eprintln!("Error: {}", err)
    }
}

fn get_puzzle_input(file_path: &str) -> Result<BufReader<File>, Error> {
    let current_dir = env::current_dir()?;
    let path = file_path;
    let absolute_path = current_dir.join(path);
    println!("{:?}", absolute_path);
    let file = File::open(absolute_path)?;

    // Create a BufReader to efficiently read the file line by line
    let reader = BufReader::new(file);
    Ok(reader)
}
