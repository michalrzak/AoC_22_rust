/*
    CODE DOES NOT RUN!

    unfortunately I was not able to generate rust code in this attempt. I tried to make a tree like
    structure, which however turned out not possible in the way I approached it.

    In the end I just wrote a python script with the same functionality as was intended in this script
    to solve the exercise
 */

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Default)]
struct Directory {
    file_list: Vec<(String, u32)>,
    directory_list: HashMap<String, Directory>,
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_07.txt").expect("File read successfully");

    let mut root = Directory::default();
    let mut current_path:Vec<&mut Directory> = Vec::from([&mut root]);

    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    for line in lines {
        let command_line = line.split(" ").collect::<Vec<&str>>();

        if command_line[0] == "$" { // is command
            let command = &command_line[1..command_line.len()];

            match command[0] {
                "cd" => {
                    /*
                        HERE IS THE PROBLEM

                        in order to push my directory to the stack, there need to be 2 instances of
                        a mutable reference to this directory. (one in the stack and one in current_dir)

                        Rust does not allow this
                     */

                    //let mut new_dir = current_path.last_mut().expect("Data contained")
                       // .directory_list.get_mut(command[1]).expect("present");
                    //current_path.push(new_dir);
                },
                "ls" => (),
                _ => assert!(false)
            }
        } else {
            match command_line[0] {
                "dir" => {
                    //current_path.last().expect("Data contained")
                     //   .directory_list.insert(command_line[1].to_string(), Directory::default());
                    ()
                }
                _ => {
                    let file_size: u32 = command_line[0].parse::<u32>().expect("file size converted successfully");
                    //current_path.last().expect("Data contained")
                   //     .file_list.push((command_line[1].to_string(), file_size));
                    ()
                }
            }
        }
    }


    println!("test");
}