use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn is_over_max(set: &[&str], maximums: HashMap<&str, i32>) -> bool {
        println!("is over max: {:?}", set);
        let color = set.last().unwrap();
        let number = set.first().unwrap().parse::<i32>().unwrap();
        let max = maximums.get(color).unwrap();
        return number > *max;
}

pub(crate) fn day_two_part_one() -> Result<(), std::io::Error> {
    let reader = crate::get_puzzle_input("src/day_two_puzzle_input.txt")?;

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

pub(crate) fn day_two_part_two() -> Result<(), std::io::Error> {
    let reader = crate::get_puzzle_input("src/day_two_puzzle_input.txt")?;

    let mut powers: Vec<i32> = vec![];

    for line_result in reader.lines() {
        let mut line = line_result?;

        let mut first_parts: Vec<_> = line.split(':').collect();
        let id = first_parts.first().unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();

        let sets: Vec<_> = first_parts.last().unwrap().trim().split(';').flat_map(|x|x.split(", ").map(str::trim)).map(|x|x.split(" ").collect::<Vec<_>>()).collect();
        let mut max_color = HashMap::new();

        max_color.insert("red", -1);
        max_color.insert("green", -1);
        max_color.insert("blue", -1);

        for set in sets {
            let color = set.last().unwrap();
            let number = set.first().unwrap().parse::<i32>().unwrap();
            let current_maximum = max_color.get(color).unwrap().clone();
            if number > current_maximum {
                max_color.insert(color, number);
            }
        }

        let power = max_color.values().fold(1, |acc, &value| acc * value);
        powers.push(power)
    }
    println!("sum of powers: {}", powers.iter().sum::<i32>());

    Ok(())
}