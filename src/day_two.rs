use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_over_max(set: &[&str], maximums: HashMap<&str, i32>) -> bool {
        println!("is over max: {:?}", set);
        let color = set.last().unwrap();
        let number = set.first().unwrap().parse::<i32>().unwrap();
        let max = maximums.get(color).unwrap();
        return number > *max;
}

pub(crate) fn day_two_part_one() -> Result<(), std::io::Error> {
    println!("day two prart one");

    let current_dir = env::current_dir()?;
    let path = "src/day_two_puzzle_input.txt";
    let absolute_path = current_dir.join(path);
    println!("{:?}", absolute_path);
    let file = File::open(absolute_path)?;

    // Create a BufReader to efficiently read the file line by line
    let reader = BufReader::new(file);
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut max_color = HashMap::new();

    // Populate the HashMap with word-to-number mappings
    max_color.insert("red", 12);
    max_color.insert("green", 13);
    max_color.insert("blue", 14);

    let mut ids: HashSet<i32> = HashSet::new();
    let mut invalid_ids: HashSet<i32> = HashSet::new();

    for line_result in reader.lines() {
        let mut line = line_result?;
        println!("{}", line);
        let mut first_parts: Vec<_> = line.split(':').collect();
        let id = first_parts.first().unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        ids.insert(id);

        let sets = first_parts.last().unwrap().trim().split(';').flat_map(|x|x.split(", ").map(str::trim)).map(|x|x.split(" ").collect::<Vec<_>>());
        let tested = sets.map(|x| is_over_max(&x, max_color.clone()));
        let is_invalid = tested.into_iter().any(|x|x);
        if (is_invalid) {
            invalid_ids.insert(id);
            println!("line: {} -> id: {:?} is_invalid: {:?}", line, id, is_invalid);
        }

    }

    let valid_ids_sum: i32 = ids.clone().into_iter().filter(|x| !invalid_ids.contains(x)).sum();
    println!("{}", valid_ids_sum);

    Ok(())
}