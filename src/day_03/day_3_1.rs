use std::collections::HashMap;
use std::fs;

fn to_score(input: char) -> u32 {
    if input.is_lowercase() {
        return input as u32 - 'a' as u32 + 1;
    }

    return input as u32 - 'A' as u32 + 27;
}

#[allow(dead_code)]
pub fn solve() {

    let input = fs::read_to_string("inputs/input_03.txt").expect("File was successfully read");

    let score: u32 = input.split('\n')
        .map(|rucksack: &str| {
            let mut items:HashMap<char, u32> = HashMap::new();
            let compartments = rucksack.split_at(rucksack.len() / 2);

            compartments.0.chars().for_each(|item| {items.entry(item).or_insert(0);});

            for item in compartments.1.chars() {
                if items.contains_key(&item) {
                    return item;
                }
            }
            print!("oof");
            return '(';
        }).map(|item| to_score(item))
        .sum();
    println!("{}", score);
}