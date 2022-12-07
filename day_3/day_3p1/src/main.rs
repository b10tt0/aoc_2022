use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let map = get_map(); // our hash map of (item:value)
    let mut sum = 0; // sum of repeated priorities
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            let rucksack = line // one rucksack per line
                .as_ref()
                .expect("couldn't read line")
                .split_at(line.as_ref().unwrap().chars().count() / 2);
            let pocket1 = rucksack.0; // one pocket per rucksack half
            let pocket2 = rucksack.1; // other half of line
            let mut pocket1_items = HashMap::new(); // map of this rucksacks items so no repeats
            let mut pocket2_items = HashMap::new();
            // mapping
            for item in pocket1.chars() {
                pocket1_items
                    .entry(item)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            for item in pocket2.chars() {
                pocket2_items
                    .entry(item)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            for item in pocket1_items {
                // check for repeats and add their value to sum
                if pocket2_items.contains_key(&item.0) {
                    sum += map.get(&item.0).unwrap();
                    // println!("repeat: {}", item.0);
                }
            }
        }
        println!("the sum of repeated item priorities is: {}", sum);
    }
}

/* make hashmap ITEM:VALUE of a:1 b:2 c:3 ... Z:52 */
fn get_map() -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for (i, item) in ITEMS.chars().enumerate() {
        map.insert(item, i + 1);
    }
    map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
