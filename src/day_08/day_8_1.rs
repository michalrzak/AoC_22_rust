use std::fs;

#[allow(dead_code)]
fn is_visible(trees:&Vec<Vec<u32>>, pos:(usize, usize), direction:&(i32, i32)) -> bool{
    let current_height = trees[pos.1][pos.0];

    let mut not_edge = true;
    let mut cx = (pos.0 as i32 + direction.0) as usize;
    let mut cy = (pos.1 as i32 + direction.1) as usize;

    while not_edge {
        let new_height = trees[cy][cx];
        if new_height >= current_height{
            return false;
        }

        if cx == 0 || cx == trees[0].len()-1 || cy == 0 || cy == trees.len()-1 {
            not_edge = false;
            break;
        }

        cx = (cx as i32 + direction.0) as usize;
        cy = (cy as i32 + direction.1) as usize;
    }

    println!("is visble");

    return true;
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_08.txt").expect("File read successfully");

    let trees = input.split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let dirs:Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut visible_count:u32 = 0;

    for y in 1..trees.len()-1 {
        for x in 1..trees[0].len()-1 {

            println!("{}, {}", x, y);

            for direction in &dirs {
                if is_visible(&trees, (x, y), direction) {
                    visible_count += 1;
                    break;
                }
            }

        }
    }

    // add edges
    visible_count += (trees.len() * 2 + (trees[0].len() - 2) * 2) as u32;

    println!("{}", visible_count);
}