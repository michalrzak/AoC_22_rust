use std::fs;

#[allow(dead_code)]
fn all_different(c1: char, c2: char, c3: char, c4: char) -> bool {
    if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
        return true;
    }
    return false;
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_06.txt").expect("File read successfully");

    let datastream: Vec<char> = input.trim().chars().collect::<Vec<char>>();

    for i in 0..datastream.len() - 4 {
        if all_different(datastream[i], datastream[i+1], datastream[i+2], datastream[i+3]) {
            println!("{}", i+4);
            return;
        }
    }

    println!("This should never have happened!");
}