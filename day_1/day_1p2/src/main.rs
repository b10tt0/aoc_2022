use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut calories: u64 = 0;
    let mut max_cals: [u64; 3] = [0, 0, 0];

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if !line.as_ref().unwrap().is_empty() {
                calories += line.as_ref().unwrap().parse::<u64>().unwrap();
            } else {
                // check if it is carrying more calories than the top 3 amounts
                if calories >= max_cals[0] {
                    max_cals[2] = max_cals[1];
                    max_cals[1] = max_cals[0];
                    max_cals[0] = calories;
                } else if calories >= max_cals[1] {
                    max_cals[2] = max_cals[1];
                    max_cals[1] = calories;
                } else if calories >= max_cals[2] {
                    max_cals[2] = calories;
                }
                // if we have finished counting calories for an elf return calories to 0
                calories = 0;
            }
        }
        // one last check for last value in buffer
        if calories > max_cals[0] {
            max_cals[1] = max_cals[0];
            max_cals[0] = calories;
        } else if calories > max_cals[1] {
            max_cals[2] = max_cals[1];
            max_cals[1] = calories;
        } else if calories > max_cals[2] {
            max_cals[2] = calories;
        }
    }
    println!("top 3 cals:");
    for (i, cal) in max_cals.iter().enumerate() {
        println!("{}: {}", i + 1, cal);
    }
    println!("sum: {}", max_cals[0] + max_cals[1] + max_cals[2]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
