use std::collections::{HashMap, HashSet};
use std::fs;

#[allow(dead_code)]
fn get_manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32{
    return (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs();
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_15.txt").expect("File read successfully");

    let mut checked: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let desired_y = 0 ..= 4000000;

    let lines = input.lines();

    for line in lines {
        println!(":)");
        let words = line.split(" ").collect::<Vec<&str>>();

        let x_sensor = words[2].split("=").last().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        let y_sensor = words[3].split("=").last().unwrap().trim_end_matches(":").parse::<i32>().unwrap();

        let x_beacon = words[8].split("=").last().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        let y_beacon = words[9].split("=").last().unwrap().parse::<i32>().unwrap();

        let distance = get_manhattan_distance((x_sensor, y_sensor), (x_beacon, y_beacon));

        let y_low = if y_sensor - distance < 0 { 0 } else { y_sensor - distance };
        let y_high = if y_sensor + distance > 4000000 { 4000000 } else { y_sensor + distance };

        for y in y_low ..= y_high {
            let x_num = (distance - (y_sensor - y).abs()) * 2 + 1;

            if !checked.contains_key(&y) {
                checked.insert(y, Vec::new());
            }
            let x_low = if x_sensor - x_num/2 < 0 { 0 } else { x_sensor - x_num/2 };
            let x_high = if x_sensor + x_num/2 > 4000000 { 4000000 } else { x_sensor + x_num/2 };

            checked.get_mut(&y).unwrap().push((x_low, x_high));
        }
    }

    let mut res_x: i64 = 0;
    let mut res_y: i64 = 0;

    for y in desired_y {
        let line = checked.get_mut(&y).unwrap();
        line.sort_by(|ele1, ele2| ele1.0.cmp(&ele2.0));

        if line[0].0 != 0 {
            println!("ALARM: {}", line[0].0);
        }

        let mut big_range = line[0];

        for i in 1 .. line.len() {
            if big_range.1 < line[i].0 {
                println!("ALARM during, {} - {}", big_range.1, line[i].0);
                println!("y was: {}", y);

                res_x = (big_range.1 + 1) as i64;
                res_y = y as i64;
                break;
            }

            if big_range.1 > line[i].1 {
                continue;
            }

            big_range = (big_range.0, line[i].1);
        }

        if big_range.1 != 4000000 {
            println!("ALARM UP, {}", big_range.1);
            break;
        }
    }

    println!("res: {}", res_x * 4000000 + res_y);

}