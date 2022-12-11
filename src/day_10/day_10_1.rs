use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_10.txt").expect("File read successfully");
    let lines = input.split("\n");

    let mut x: i32 = 1;

    let cycle_offset: u32 = 20;
    let mut cycle: u32 = cycle_offset + 1;

    let mut res: i64 = 0;
    for line in lines {
        let command = line.split(" ").collect::<Vec<&str>>();
        if command[0] == "addx" {
            cycle += 1;
            println!("Used i={} and x={}", (cycle - cycle_offset), x);
            if cycle % 40 == 0 {
                res += (cycle - cycle_offset) as i64 * x as i64;
                // println!("Used i={} and x={}", (cycle - cycle_offset), x);
            }

            x += command[1].parse::<i32>().unwrap();
        }

        cycle += 1;
        println!("Used i={} and x={}", (cycle - cycle_offset), x);

        if cycle % 40 == 0 {
            res += (cycle - cycle_offset) as i64 * x as i64;
            // println!("Used i={} and x={}", (cycle - cycle_offset), x);
        }

        //println!("{}: x={}", cycle, x);
    }

    println!("{}", res);

}