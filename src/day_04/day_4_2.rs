use std::fs;

#[allow(dead_code)]
fn is_in_range(value:u32, range:&Vec<u32>) -> bool {
    if value >= range[0] && value <= range[1] {
        return true;
    }

    return false;
}

#[allow(dead_code)]
pub fn solve() {
    let input:String = fs::read_to_string("inputs/input_04.txt").expect("File read successfully");

    let ranges:Vec<Vec<Vec<u32>>> = input.split("\n")
        .map(|line:&str| line.split(",")
            .map(|range:&str| range.split("-")
                .map(|value:&str| value.parse().unwrap())
                .collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>())
        .collect::<Vec<Vec<Vec<u32>>>>();

    let output:u32 = ranges.iter()
        .filter(|range| {
            println!("{}", range[0][0]);
            if is_in_range(range[0][0], &range[1]) || is_in_range(range[0][1], &range[1])
                || is_in_range(range[1][0], &range[0]) || is_in_range(range[1][1], &range[0])
            {
                return true;
            }
            return false;
        }).count() as u32;

    println!("----{}-----", output);
}