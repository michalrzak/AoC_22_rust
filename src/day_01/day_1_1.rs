use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let input = fs::read_to_string("inputs/input_1.txt").expect("File was successfully read");
    let elves = input.split("\n\n").map(|calories| calories.split('\n')
        .map(|calory| calory.parse::<i32>().expect("Successfully converted calory to integer")).sum::<i32>()).collect::<Vec<i32>>();

    let mut max_elf:i32 = 0;
    for elf in elves {
        if elf > max_elf {
            max_elf = elf;
        }
    }
    println!("{}", max_elf);
}