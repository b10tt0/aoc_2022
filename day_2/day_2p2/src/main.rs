use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

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
            let their_move = get_move(line.as_ref().unwrap().chars().nth(0).unwrap());
            let ldw = win_check(line.as_ref().unwrap().chars().nth(2).unwrap());
            let my_move = decision((their_move, ldw));
            my_score += ldw + my_move;
        }
    }
    println!("your total score: {}", my_score);
}

fn decision((their_move, ldw): (u32, u32)) -> u32 {
    let my_move = match (their_move, ldw) {
        (PAPER, LOSE) | (ROCK, DRAW) | (SCISSORS, WIN) => ROCK,
        (SCISSORS, LOSE) | (PAPER, DRAW) | (ROCK, WIN) => PAPER,
        (ROCK, LOSE) | (SCISSORS, DRAW) | (PAPER, WIN) => SCISSORS,
        _ => panic!("panic in decision"),
    };
    my_move
}

fn win_check(c: char) -> u32 {
    let ldw = match c {
        'X' => LOSE,
        'Y' => DRAW,
        'Z' => WIN,
        _ => panic!("panic in win_check"),
    };
    ldw
}

// check what move they make
fn get_move(c: char) -> u32 {
    let mv = match c {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
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
