use std::collections::{HashSet, LinkedList};
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

fn get_movment(source: &(i32, i32), dest: &(i32, i32)) -> (i32, i32) {
    let mut diff = ((dest.0 - source.0) , (dest.1 - source.1));
    if diff.0.abs() == 2 {
        diff.0 = diff.0 / 2;
    }

    if diff.1.abs() == 2 {
        diff.1 = diff.1 / 2;
    }

    return diff;
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_09.txt").expect("File read successfully");
    let lines = input.split("\n");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = Vec::new();
    for i in 0..10 {
        rope.push((0, 0))
    }
    visited.insert(rope.last().unwrap().clone());

    for line in lines {
        let command = line.split(" ").collect::<Vec<&str>>();
        let steps: i32 = command[1].parse::<i32>().unwrap();
        match command[0] {
            "R" => {
                let new_x = rope[0].0 + steps;
                for x in (rope[0].0 + 1)..=new_x {
                    rope[0].0 = x;

                    for i in 1..rope.len() {
                        if !touching(rope[i - 1], rope[i]) {
                            let movement = get_movment(&rope[i], &rope[i - 1]);
                            println!("[[[[{} {}", movement.0, movement.1);
                            rope[i] = (rope[i].0 + movement.0, rope[i].1 + movement.1);
                        }
                    }

                    println!("+++++++++++++++++++++");

                    for ele in &rope {
                        println!("{} {}", ele.0, ele.1);
                    }
                    visited.insert(*rope.last().unwrap());
                }
            }
            "L" => {
                let new_x = rope[0].0 - steps;
                for x in (new_x..rope[0].0).rev() {
                    rope[0].0 = x;

                    for i in 1..rope.len() {
                        if !touching(rope[i - 1], rope[i]) {
                            let movement = get_movment(&rope[i], &rope[i - 1]);
                            rope[i] = (rope[i].0 + movement.0, rope[i].1 + movement.1);
                        }
                    }
                    visited.insert(*rope.last().unwrap());
                }
            }
            "U" => {
                let new_y = rope[0].1 - steps;
                for y in (new_y..rope[0].1).rev() {
                    rope[0].1 = y;

                    for i in 1..rope.len() {
                        if !touching(rope[i - 1], rope[i]) {
                            let movement = get_movment(&rope[i], &rope[i - 1]);
                            println!("[[[[{} {}", movement.0, movement.1);

                            rope[i] = (rope[i].0 + movement.0, rope[i].1 + movement.1);
                        }
                    }
                    println!("+++++++++++++++++++++");

                    for ele in &rope {
                        println!("{} {}", ele.0, ele.1);
                    }
                    visited.insert(*rope.last().unwrap());
                }
            }
            "D" => {
                let new_y = rope[0].1 + steps;
                for y in (rope[0].1 + 1)..=new_y {
                    rope[0].1 = y;

                    for i in 1..rope.len() {
                        if !touching(rope[i - 1], rope[i]) {
                            let movement = get_movment(&rope[i], &rope[i - 1]);
                            rope[i] = (rope[i].0 + movement.0, rope[i].1 + movement.1);
                        }
                    }
                    visited.insert(*rope.last().unwrap());
                }
            }
            _ => assert!(false)
        }

        println!("---------------");

        for ele in &rope {
            println!("{} {}", ele.0, ele.1);
        }
    }

    for ele in &visited {
        println!("{} {}", ele.0, ele.1);
    }

    println!("RESULT: {}", visited.len());

    println!("---------------");
    println!("---------------");


    for ele in rope {
        println!("{} {}", ele.0, ele.1);
    }
}