use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// rock paper scissors, best out of 3
const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

fn main() {
    let mut my_score = 0;
    // let mut their_score = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            let (them, me) = (
                move_check(line.as_ref().unwrap().chars().nth(0).unwrap()),
                move_check(line.as_ref().unwrap().chars().nth(2).unwrap()),
            );
            let wld = win_check((them, me));
            my_score += wld + me;
        }
    }
    println!("your total score: {}", my_score);
}

/* rules:
 * rock beats scissors
 * paper beats rock
 * scissors beats paper
 */
fn win_check((them, me): (u32, u32)) -> u32 {
    let wld = match (them, me) {
        (SCISSORS, ROCK) | (PAPER, SCISSORS) | (ROCK, PAPER) => WIN,
        (ROCK, SCISSORS) | (SCISSORS, PAPER) | (PAPER, ROCK) => LOSE,
        (ROCK, ROCK) | (SCISSORS, SCISSORS) | (PAPER, PAPER) => DRAW,
        _ => panic!("panic in win_check"),
    };
    wld
}

fn move_check(c: char) -> u32 {
    let mv = match c {
        'A' | 'X' => ROCK,
        'B' | 'Y' => PAPER,
        'C' | 'Z' => SCISSORS,
        _ => panic!("panic in move_check"),
    };
    mv
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
