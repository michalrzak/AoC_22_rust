use std::borrow::Borrow;
use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let input:String = fs::read_to_string("inputs/input_05.txt").expect("File read successfully");

    let mut iter = input.splitn(2, "\n\n");
    let current_crates: &str = iter.next().unwrap();
    let moves: &str = iter.next().unwrap();

    let mut current_crates_lines: Vec<&str> = current_crates.split("\n").collect::<Vec<&str>>();
    let num_of_stacks: usize = current_crates_lines.last().unwrap().split("   ").collect::<Vec<&str>>().len();
    current_crates_lines.remove(current_crates_lines.len()-1);

    let mut ship:Vec<Vec<char>> = (0..num_of_stacks).map(|ele| Vec::new()).collect::<Vec<Vec<char>>>();

    for line in current_crates_lines.iter().rev() {
        let mut line_it = line.chars();

        for i in 0..num_of_stacks {
            let curr = line_it.nth(1).unwrap_or_else(||' ');
            if curr != ' ' {
                ship[i].push(curr);
                println!("{}, {}",i, curr);
            }
            line_it.nth(1);
        }
    }

    let moves_lines = moves.split("\n");

    for line in moves_lines {
        let moves_words = line.split(" ").collect::<Vec<&str>>();
        let move_count: u32 = moves_words[1].parse::<u32>().unwrap();
        let source: usize = moves_words[3].parse::<usize>().unwrap() - 1;
        let dest: usize = moves_words[5].parse::<usize>().unwrap() - 1;

        let mut crane_arm: Vec<char> = Vec::new();
        for i in 0..move_count {
            let val = ship[source].pop().unwrap();
            println!("{} to {}", val, dest);
            crane_arm.push(val);
        }
        for val in crane_arm.iter().rev() {
            ship[dest].push(*val);
        }
    }

    for elf_crates in ship {
        print!("{}", elf_crates.last().unwrap());
    }

    println!("")


}