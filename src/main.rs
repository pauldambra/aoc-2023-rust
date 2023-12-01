mod day_one;
use day_one::day_one;
fn main() {
    if let Err(err) = day_one() {
        eprintln!("Error: {}", err);
    }
}
