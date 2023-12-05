use std::collections::HashMap;
use std::io::BufRead;
use itertools::Itertools;

pub(crate) fn day_four_part_one() -> Result<(), std::io::Error> {
    let reader = crate::get_puzzle_input("src/day_four_puzzle_input.txt")?;

    let mut scores = HashMap::new();

    let lines = reader.lines().map(|l| l.unwrap());

    for read_line in lines {
        let (game, winning_numbers, card_numbers) = as_card_numbers(read_line);
        let score = score_matches(&winning_numbers, &card_numbers);
        scores.insert(game, score);
    }

    println!("total score is {}", scores.values().sum::<i32>());
    Ok(())
}

pub(crate) fn day_four_part_two() -> Result<(), std::io::Error> {
    let reader = crate::get_puzzle_input("src/day_four_puzzle_input.txt")?;

    let mut scores = HashMap::new();

    let lines = reader.lines().map(|l| l.unwrap());

    for read_line in lines {
        let (game, winning_numbers, card_numbers) = as_card_numbers(read_line);
        let score = count_matches(&winning_numbers, &card_numbers);
        scores.insert(game, score);
    }

    let mut instance_counts = HashMap::new();

    let max_game_id = scores.keys().max().unwrap();
    // println!("Scores = {:?}", scores);
    for game in scores.keys().sorted() {
        let score = scores.get(game).unwrap();
        let copies = *instance_counts.get(game).unwrap_or(&1);
        println!("game {} has {} copies", game, copies);
        // add a copy for each of these game ids
        let won_copy_range: std::ops::Range<i32> = std::ops::Range { start: *game, end: (game + score + 1) };

        for game_id in won_copy_range.clone() {
            if (game_id >= *max_game_id + 1) { // wat why + 1
                println!("game {} does not exist - skipping", game_id);
                continue;
            }
            match instance_counts.get(&game_id.clone()) {
                None => {
                    if game_id > 1 {
                        // println!("first time seeing - {}", game_id);
                        instance_counts.insert(game_id, copies);
                    }
                }
                Some(count) => {
                    if game_id > 1 {
                        // println!("game id {} has {} - setting {}", game_id, count, count + 1);
                        instance_counts.insert(game_id, count + copies);
                    }
                }
            }
            // println!("{:?}", instance_counts);
        }
    }
    // 5670274 is too low
    // 11666129 is too high
    println!("so you have {:?}", instance_counts.values().sum::<i32>());
    Ok(())
}

fn as_card_numbers(read_line: String) -> (i32, Vec<i32>, Vec<i32>) {
    let splat: Vec<_> = read_line.split(':').map(str::trim).collect();
    let game = splat.first().unwrap().split(' ').last().unwrap().parse::<i32>().unwrap();

    let numbers = splat.last().unwrap();
    let splot: Vec<_> = numbers.split("|").map(str::trim).map(|x| x.split(' ').map(str::trim).filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect();
    let winning_numbers = splot.first().unwrap();
    let card_numbers = splot.last().unwrap();
    (game, winning_numbers.clone(), card_numbers.clone())
}

fn score_matches(winning_numbers: &Vec<i32>, card_numbers: &Vec<i32>) -> i32 {
    let mut match_count = 0;
    let mut score = 0;
    for wn in winning_numbers {
        if card_numbers.contains(wn) {
            match_count += 1;
            if score == 0 {
                score = 1;
            } else {
                score = score * 2;
            }
        }
    }
    score
}

fn count_matches(winning_numbers: &Vec<i32>, card_numbers: &Vec<i32>) -> i32 {
    let mut match_count = 0;
    let mut score = 0;
    for wn in winning_numbers {
        if card_numbers.contains(wn) {
            match_count += 1;
        }
    }
    match_count
}