use std::collections::HashSet;
use std::fs;

#[allow(dead_code)]
fn touching(head: (i32, i32), tail: (i32, i32)) -> bool {

    for x_i in -1..=1 {
        for y_i in -1..=1 {
            if head.0 + x_i == tail.0 && head.1 + y_i == tail.1 {
                return true;
            }
        }
    }

    return false;
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_09.txt").expect("File read successfully");
    let lines = input.split("\n");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current_head: (i32, i32) = (0, 0);
    let mut current_tail:(i32, i32) = (0, 0);
    visited.insert(current_tail);

    for line in lines {
        let command = line.split(" ").collect::<Vec<&str>>();
        let steps: i32 = command[1].parse::<i32>().unwrap();
        match command[0] {
            "R" => {
                let new_x = current_head.0 + steps;
                for x in (current_head.0 + 1)..=new_x {
                    current_head.0 = x;
                    println!("HEAD: {}, {}", current_head.0, current_head.1);
                    println!("TAIL: {}, {}", current_tail.0, current_tail.1);
                    if !touching(current_head, current_tail) {
                        current_tail = (x-1, current_head.1);
                        visited.insert(current_tail);
                        println!("R: {}, {}", current_tail.0, current_tail.1);
                    }
                }
            },
            "L" => {
                let new_x = current_head.0 - steps;
                for x in (new_x..current_head.0).rev() {
                    current_head.0 = x;
                    println!("HEAD: {}, {}", current_head.0, current_head.1);
                    println!("TAIL: {}, {}", current_tail.0, current_tail.1);

                    if !touching(current_head, current_tail) {
                        current_tail = (x+1, current_head.1);
                        visited.insert(current_tail);
                        println!("L: {}, {}", current_tail.0, current_tail.1);

                    }
                }
            },
            "U" => {
                let new_y = current_head.1 - steps;
                for y in (new_y..current_head.1).rev() {
                    current_head.1 = y;
                    println!("HEAD: {}, {}", current_head.0, current_head.1);
                    println!("TAIL: {}, {}", current_tail.0, current_tail.1);

                    if !touching(current_head, current_tail) {
                        current_tail = (current_head.0, y+1);
                        visited.insert(current_tail);
                        println!("U: {}, {}", current_tail.0, current_tail.1);
                    }
                }
            },
            "D" => {
                let new_y = current_head.1 + steps;
                for y in (current_head.1 + 1)..=new_y {
                    current_head.1 = y;
                    println!("HEAD: {}, {}", current_head.0, current_head.1);
                    println!("TAIL: {}, {}", current_tail.0, current_tail.1);

                    if !touching(current_head, current_tail) {
                        current_tail = (current_head.0, y-1);
                        visited.insert(current_tail);
                    }
                }
            },
            _ => assert!(false)
        }
    }

    for ele in &visited {
        println!("{} {}", ele.0, ele.1);
    }
    println!("{}", visited.len());
}