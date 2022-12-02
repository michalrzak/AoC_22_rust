use std::collections::HashMap;
use std::fs;


fn decode(input: char) -> char {
    let x_repr: u32 = 'X' as u32;
    let a_repr: u32 = 'A' as u32;

    let char_shift: u32 = x_repr - a_repr;

    let new_repr: u32 = input as u32 - char_shift;

    return char::from_u32(new_repr).expect("Successfully converted from u32");
}

fn to_score(input: char) -> u32 {
    return input as u32 - 'A' as u32 + 1;
}

#[allow(dead_code)]
pub fn solve() {

    let winning = HashMap::from([
        ('A', 'C'),
        ('B', 'A'),
        ('C', 'B')
    ]);


    let input = fs::read_to_string("inputs/input_02.txt").expect("File was successfully read");

    let rps_score: u32 = input.split('\n').map(|rps_pairs:&str|
        rps_pairs.split(' ').map(|rps| rps.chars().next().expect("Not empty")).collect::<Vec<char>>())
        .map(|rps_pair: Vec<char>| (rps_pair[0], decode(rps_pair[1])))
        .map(|rps_pair: (char, char)| {
            let selected_score: u32 = to_score(rps_pair.1);

            if winning[&rps_pair.1] == rps_pair.0 {
                return 6 + selected_score;
            }

            if rps_pair.1 == rps_pair.0 {
                return 3 + selected_score;
            }

            return selected_score;
        })
        .sum();

    println!("{}", rps_score);
}