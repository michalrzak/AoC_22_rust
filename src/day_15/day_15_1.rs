use std::collections::HashSet;
use std::fs;

#[allow(dead_code)]
fn get_manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32{
    return (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs();
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_15.txt").expect("File read successfully");

    let mut checked: HashSet<(i32, i32)> = HashSet::new();
    let desired_y = 2000000;

    let lines = input.lines();

    for line in lines {
        let words = line.split(" ").collect::<Vec<&str>>();

        let x_sensor = words[2].split("=").last().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        let y_sensor = words[3].split("=").last().unwrap().trim_end_matches(":").parse::<i32>().unwrap();

        let x_beacon = words[8].split("=").last().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        let y_beacon = words[9].split("=").last().unwrap().parse::<i32>().unwrap();
        
        let distance = get_manhattan_distance((x_sensor, y_sensor), (x_beacon, y_beacon));

        let mut x_num = 1;
        let mut x_num_dif = 2;

        let y_range = y_sensor - distance ..= y_sensor + distance;
        if !y_range.contains(&desired_y) {
            continue;
        }


        //for y in y_sensor - distance ..= y_sensor + distance {
        let y = desired_y;
        // if x_num == distance * 2 + 1 {
        //     x_num_dif = -x_num_dif;
        // }
        x_num = (distance - (y_sensor - y).abs()) * 2 + 1;

        for x in x_sensor - x_num/2 ..= x_sensor + x_num/2 {
            if x == x_beacon && y == y_beacon || x == x_sensor && y == y_sensor {
                continue;
            }

            checked.insert((x, y));
        }
        x_num += x_num_dif;
        //}
    }

    // let x_min = checked.iter().min_by(|ele1, ele2| ele1.0.cmp(&ele2.0)).unwrap().0;
    // let x_max = checked.iter().max_by(|ele1, ele2| ele1.0.cmp(&ele2.0)).unwrap().0;
    //
    // let mut res: usize = 0;
    // for x in x_min .. x_max {
    //     if checked.contains(&(x, desired_y)) {
    //         res += 1;
    //         print!("#");
    //     } else {
    //         print!(".")
    //     }
    // }
    // println!();

    let res = checked.iter().filter(|ele| ele.1 == desired_y).count();
    checked.iter().filter(|ele| ele.1 == desired_y).for_each(|ele| println!("{}, {}", ele.0, ele.1));


    println!("{}", res);

}