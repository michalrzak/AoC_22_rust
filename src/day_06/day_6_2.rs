use std::fs;

#[allow(dead_code)]
fn all_different(inputs: Vec<char>) -> bool {
    for i in 0..inputs.len() {
        let curr: char = inputs[i];
        for j in (i + 1)..inputs.len() {
            if curr == inputs[j] {
                return false;
            }
        }
    }

    return true;
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_06.txt").expect("File read successfully");

    let datastream: Vec<char> = input.trim().chars().collect::<Vec<char>>();

    for i in 0..datastream.len() - 14 {
        if all_different(datastream[i..i + 14].to_vec()) {
            println!("{}", i + 14);
            return;
        }
    }

    println!("This should never have happened!");
}