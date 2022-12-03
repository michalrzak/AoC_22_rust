use std::collections::{HashMap, HashSet};
use std::fs;

fn to_score(input: char) -> u32 {
    println!("{}", input);
    if input == '_' {
        return 0;
    }

    if input.is_lowercase() {
        return input as u32 - 'a' as u32 + 1;
    }

    return input as u32 - 'A' as u32 + 27;
}

#[allow(dead_code)]
pub fn solve() {

    let input = fs::read_to_string("inputs/input_03.txt").expect("File was successfully read");

    let mut index:u32 = 0;
    let mut items:HashSet<char> = HashSet::new();


    let score: u32 = input.split('\n')
        .map(|rucksack: &str| {
            if index == 0 {
                rucksack.chars().for_each(|item| {
                    items.insert(item);
                });
            } else if index == 1 {
                let current_items = rucksack.chars().collect::<HashSet<char>>();
                items = current_items.intersection(&items)
                    .map(|item| *item)
                    .collect::<HashSet<char>>();
            } else {
                for item in rucksack.chars() {
                    if items.contains(&item) {
                        index = (index + 1) % 3;
                        items = HashSet::new();
                        return item;
                    }
                }
            }
            index = (index + 1) % 3;

            return '_';
        }).map(|item| to_score(item))
        .sum();
    println!("{}", score);
}