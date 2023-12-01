use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day_one() -> Result<(), std::io::Error> {
    let mut word_to_number = HashMap::new();

    // Populate the HashMap with word-to-number mappings
    word_to_number.insert("one", "o1ne");
    word_to_number.insert("two", "t2wo");
    word_to_number.insert("three", "t3hree");
    word_to_number.insert("four", "f4our");
    word_to_number.insert("five", "f5ive");
    word_to_number.insert("six", "s6ix");
    word_to_number.insert("seven", "s7even");
    word_to_number.insert("eight", "e8ight");
    word_to_number.insert("nine", "n9ine");

    let needles = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let current_dir = env::current_dir()?;
    let path = "src/day_one_puzzle_input.txt";
    let absolute_path = current_dir.join(path);
    println!("{:?}", absolute_path);
    let file = File::open(absolute_path)?;

    // Create a BufReader to efficiently read the file line by line
    let reader = BufReader::new(file);

    let mut numeric_values: Vec<u32> = Vec::new();

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        let mut line = line_result?;

        for needle in &needles {
            if let Some(replacement) = word_to_number.get(*needle) {
                line = line.replace(needle, replacement);
            }
        }

        println!("{}", line);

        let numeric_chars: Vec<char> = line.chars().filter(|&c| c.is_numeric()).collect();

        let first_numeric = numeric_chars.first();

        let last_numeric = numeric_chars.last();

        // let mut first_numeric = String::new();
        // let mut last_numeric = String::new();
        let mut combined_numeric = String::new();
        //
        // match word_to_number.get(min_needle) {
        //     Some(number) => {
        //         first_numeric = number.to_string();
        //     }
        //     None => println!("No mapping found for {}", min_needle),
        // }
        // match word_to_number.get(max_needle) {
        //     Some(number) => {
        //         last_numeric = number.to_string();
        //     }
        //     None => println!("No mapping found for {}", max_needle),
        // }

        combined_numeric.push(*first_numeric.unwrap());
        combined_numeric.push(*last_numeric.unwrap());
        numeric_values.push(combined_numeric.parse::<u32>().unwrap());
        let mut first_numeric: Option<char> = None;
        // let mut last_numeric: Option<char> = None;
        // let mut combined_numeric = String::new();
        //
        // for needle in needles {
        //     if character.is_numeric() {
        //         first_numeric = Some(character);
        //         break; // Stop iterating once we find the first numeric character
        //     }
        // }
        // for character in line.chars().rev() {
        //     if character.is_numeric() {
        //         last_numeric = Some(character);
        //         break; // Stop iterating once we find the last numeric character
        //     }
        // }
        //
        // if let (Some(first), Some(last)) = (first_numeric, last_numeric) {
        //     combined_numeric.push(first_numeric.unwrap());
        //     combined_numeric.push(last_numeric.unwrap());
        //     numeric_values.push(combined_numeric.parse::<u32>().unwrap())
        // } else {
        //     println!("No numeric characters found.");
        // }
        // println!("{}", line);
    }

    // Calculate the sum of all numeric values
    let sum: u32 = numeric_values.iter().sum();

    println!("Numeric Values: {:?}", numeric_values);
    println!("Sum of Numeric Values: {}", sum);

    Ok(())
}
