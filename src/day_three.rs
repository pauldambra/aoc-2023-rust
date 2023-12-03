use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::ops::Mul;
use crate::aoc;
use itertools::Itertools;
use crate::aoc::MultiXCoord;

pub(crate) fn day_three_part_one() -> Result<(), std::io::Error> {
    let reader = crate::get_puzzle_input("src/day_three_puzzle_input.txt")?;

    let processed_schematic = process_schematic(reader)?;

    // now that we have the processed schematic we check every coordinate...
    // at each coordinate if its value is numeric,
    // we'll have the x and y and can create a coordinate to get neighbours
    // with the neighbours we look up the value in the schematic
    // any neighbour that is not numeric and not a dot means we should include the part number

    let mut part_numbers: Vec<i32> = vec![];

    for needle_y in processed_schematic.keys().sorted() {
        // println!("checking row: {}", needle_y);
        let xs = processed_schematic.get(needle_y).unwrap();
        for (needle_x, needle) in xs.iter().sorted() {
            // println!("x: {}, y: {} holds {}", needle_x, needle_y, needle);
            let number_needle = needle.val.parse::<i32>();
            match number_needle {
                Err(e) => {
                    // println!("needle is not numeric so we can ignore it -- {}", needle)
                }
                Ok(number) => {
                    let cs = needle.neighbours();
                    // println!("neighbours for {}, {} are {}", needle_x, needle_y, cs.iter().join(", "));
                    let mut matched = false;
                    for nbr in cs {
                        if matched {
                            continue
                        }

                        let found_y = processed_schematic.get(&nbr.y);
                        if found_y.is_none() {
                            continue;
                        }
                        let found_x = processed_schematic.get(&nbr.y).unwrap().get(&nbr.x);
                        match found_x {
                            None => continue,
                            Some(multi_x) => {
                                let val = &multi_x.val;
                                // println!("neighbour is: {}", val);
                                if val != "." && val.parse::<i32>().is_err() {
                                    println!("found part number: {}", number);
                                    part_numbers.push(number);
                                    matched = true;
                                } else {
                                    // println!("not a part number: {}", number);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("found {}", part_numbers.iter().join(", "));
    println!("which total {:?}", part_numbers.iter().sum::<i32>());

    Ok(())
}

pub(crate) fn day_three_part_two() -> Result<(), std::io::Error> {
    let reader = crate::get_puzzle_input("src/day_three_puzzle_input.txt")?;

    let processed_schematic = process_schematic(reader)?;

    println!("2, 0 has {}", processed_schematic.get(&0).unwrap().get(&2).unwrap());

    // now that we have the processed schematic we check every coordinate...
    // at each coordinate if its value is numeric,
    // we'll have the x and y and can create a coordinate to get neighbours
    // with the neighbours we look up the value in the schematic
    // any neighbour that is not numeric and not a dot means we should include the part number

    let mut part_numbers: Vec<i32> = vec![];

    let mut gears : Vec<HashSet<&MultiXCoord>> = vec![];

    for needle_y in processed_schematic.keys().sorted() {
        // println!("checking row: {}", needle_y);
        let xs = processed_schematic.get(needle_y).unwrap();
        for (needle_x, needle) in xs.iter().sorted() {
            // println!("x: {}, y: {} holds {}", needle_x, needle_y, needle);
            let potential_gear = &needle.val;
            let mut matched_numbers = HashSet::new();
            if (potential_gear == "*") {
                println!("found potential gear at {}, {}", needle_y, needle_x);
                let cs = needle.neighbours();
                println!("neighbours for {}, {} are {}", needle_x, needle_y, cs.iter().join(", "));
                for nbr in cs {
                    let found_y = processed_schematic.get(&nbr.y);
                    if found_y.is_none() {
                        continue;
                    }
                    let found_x = processed_schematic.get(&nbr.y).unwrap().get(&nbr.x);
                    if found_x.is_none() {
                        println!("none at {}, {}", needle_x, needle_y)
                    } else {
                        println!("yay, at {}, {} there is {}", needle_x, needle_y, found_x.unwrap())
                    }
                    match found_x {
                        None => continue,
                        Some(multi_x) => {
                            let val = &multi_x.val;
                            // println!("neighbour is: {}", val);
                            if val.parse::<i32>().is_ok() {
                                println!("found part number: {}", multi_x);
                                matched_numbers.insert(multi_x);
                            } else {
                                // println!("not a part number: {}", number);
                            }
                        }
                    }
                }
                if matched_numbers.len() > 1 {
                    println!("found matched gear at {}, {}", needle_y, needle_x);
                    gears.push(matched_numbers);
                } else {
                    println!("only {} numbers for gear at {}, {}", matched_numbers.len(), needle_y, needle_x);
                }

            }
        }
    }

    // println!("found {}", part_numbers.iter().join(", "));
    println!("gear!!!");
    gears.iter().for_each(|g| {
        println!("{}", g.iter().join("||||"))
    });

    let total = gears.iter().fold(0, |acc, g| {
        let mxcs: Vec<&&MultiXCoord> = g.iter().collect();
        let left = &mxcs.first().unwrap().val.parse::<i32>().unwrap();
        let right = &mxcs.last().unwrap().val.parse::<i32>().unwrap();
        acc + (left * right)
    });
    println!("gear ratio total = {}", total);

    Ok(())
}


fn process_schematic(reader: BufReader<File>) -> Result<HashMap<i32, HashMap<i32, MultiXCoord>>, Error> {
    // process in direction we read, so Map<Y, Map<X, value>>
    let mut line_y: i32 = 0;
    let mut line_x: i32 = 0;
    let mut processed_schematic = HashMap::new();

    for line_result in reader.lines() {
        line_x = 0;
        let mut line = line_result?;
        println!("processing row: {} -- {}", line_y, line);
        let matched_y = processed_schematic.entry(line_y).or_insert_with(|| HashMap::new());

        let mut reading_number = false;
        let mut number_being_read = String::new();
        let mut started_reading_at = 0;
        for ch in line.chars() {
            if ch.is_numeric() {
                if !reading_number {
                    // println!("started reading number at x: {}", line_x);
                    started_reading_at = line_x;
                }
                reading_number = true;
                number_being_read.push(ch);
            } else {
                if reading_number {
                    reading_number = false;
                    // range is exclusive at the top end...
                    let x_range = std::ops::Range { start: started_reading_at, end: line_x };
                    let xs_for_number: Vec<_> = x_range.into_iter().collect();
                    // insert the multi coord at each of the xs so the gear lookup can find them
                    // this probably breaks part one which iterated the coords
                    xs_for_number.clone().into_iter().for_each(|x_in_range| {
                        let matched_multi_coord = aoc::MultiXCoord::new(xs_for_number.clone(), line_y, number_being_read.clone()).unwrap();
                        matched_y.insert(x_in_range, matched_multi_coord.clone());
                    });
                    number_being_read = String::new();
                }
                // println!("inserting {} at {}", ch, line_x);
                matched_y.insert(line_x, aoc::MultiXCoord::new(vec![line_x], line_y, ch.to_string()).unwrap());
            }
            line_x += 1;
        }


        if (reading_number) {
            reading_number = false;
            // range is exclusive at the top end...
            let x_range = std::ops::Range { start: started_reading_at, end: line_x };
            let xs_for_number = x_range.into_iter().collect();
            // println!("from {} to {} gives {:?}", started_reading_at, line_x, xs_for_number);
            let matched_multi_coord = aoc::MultiXCoord::new(xs_for_number, line_y, number_being_read.clone()).unwrap();
            matched_y.insert(started_reading_at, matched_multi_coord.clone());
            // println!("completing number at x: {} -- {}", started_reading_at, matched_y.get(&started_reading_at).unwrap());
            number_being_read = String::new();
        }

        line_y += 1;
    }
    Ok(processed_schematic)
}