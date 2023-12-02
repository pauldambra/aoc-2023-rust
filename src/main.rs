mod day_one;
mod day_two;

use day_one::day_one;
use day_two::day_two_part_one;

fn main() {
    if let Err(err) = day_one() {
        eprintln!("Error: {}", err);
    }
    if let Err(err) = day_two_part_one() {
        eprintln!("Error: {}", err);
    }
}
