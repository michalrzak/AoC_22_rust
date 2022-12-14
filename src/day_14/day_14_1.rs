use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_14.txt").expect("File read successfully");

    let walls: Vec<Vec<(usize, usize)>> = input.lines()
        .map(|ele| ele.split(" -> ")
            .map(|coords| coords.split(","))
            .map(|mut coords| (coords.next().unwrap().parse::<usize>().unwrap(), coords.next().unwrap().parse::<usize>().unwrap()))
            .collect::<Vec<(usize, usize)>>()).collect::<Vec<Vec<(usize, usize)>>>();

    let mut cave: HashMap<(usize,usize), char> = HashMap::new();

    // build walls
    for wall in walls {
        for i in 1..wall.len() {
            let from: &(usize, usize) = &wall[i-1];
            let to: &(usize, usize) = &wall[i];

            if from.0 == to.0 {
                assert_ne!(from.1, to.1);

                let larger: usize = if from.1 > to.1 { from.1 } else { to.1 };
                let smaller: usize = if larger == from.1 { to.1 } else { from.1 };

                for y in smaller ..= larger {
                    cave.insert((from.0, y), '#');
                }
            } else if from.1 == to.1 {
                assert_ne!(from.0, to.0);

                let larger: usize = if from.0 > to.0 { from.0 } else { to.0 };
                let smaller: usize = if larger == from.0 { to.0 } else { from.0 };

                println!("{} {}", smaller, larger);

                for x in smaller ..= larger {
                    println!("{}, {}", x, from.1);
                    cave.insert((x, from.1), '#');
                }
            } else {
                assert!(false);
            }
        }
    }

    let largest_y: usize = cave.iter().max_by_key(|ele| ele.0.1).map(|ele| ele.0.1).unwrap();

    let mut count: u32 = 0;

    let mut sand_overflow = false;
    while !sand_overflow {
        count += 1;

        let mut sand_pos: (usize, usize) = (500, 0);

        let mut sand_falling = true;
        while sand_falling {
            // check if bellow is occupied
            if cave.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
                // check left
                if !cave.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                    sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                    continue;
                }
                // check right
                if !cave.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)){
                    sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                    continue;
                }

                cave.insert(sand_pos, 'o');
                sand_falling = false;
                break;
            }

            sand_pos = (sand_pos.0, sand_pos.1 + 1);
            if sand_pos.1 > largest_y {
                sand_falling = false;
                sand_overflow = true;
                count -= 1;
                break;
            }
        }
    }

    println!("{}", count);
}