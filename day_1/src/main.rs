use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut calories: u64 = 0;
    let mut max_cals: u64 = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if line.as_ref().unwrap().is_empty() {
                // if we have finished counting calories for an elf
                // check if it is carrying more calories than the largest amount
                if calories > max_cals {
                    max_cals = calories;
                    calories = 0;
                } else {
                    calories = 0;
                }
            } else {
                calories += line.as_ref().unwrap().parse::<u64>().unwrap();
            }
        }
    }
    println!(
        "Elf with the most calories is carrying: {} calories",
        max_cals
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
