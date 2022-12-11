use std::fs;

#[allow(dead_code)]
fn draw_screen(cycle: u32, sprite: i32) {
    let screen_pos: i64 = ((cycle - 1) % 40 ) as i64; // cycle starts from 1

    if screen_pos == 0  {
        println!("");
    }

    let converted: i64 = sprite as i64;

    if screen_pos == converted || screen_pos + 1 == converted || screen_pos - 1 == converted {
        print!(".");
    } else {
        print!("#");
    }
}

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
            draw_screen(cycle - cycle_offset, x);

            x += command[1].parse::<i32>().unwrap();
        }

        cycle += 1;
        draw_screen(cycle - cycle_offset, x);
    }

    println!("{}", res);

}