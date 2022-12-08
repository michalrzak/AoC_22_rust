use std::fs;

#[allow(dead_code)]
fn visible_count(trees: &Vec<Vec<u32>>, pos: (usize, usize), direction: &(i32, i32)) -> u32 {
    let current_height = trees[pos.1][pos.0];

    let mut not_edge = true;
    let mut cx = (pos.0 as i32 + direction.0) as usize;
    let mut cy = (pos.1 as i32 + direction.1) as usize;

    let mut count: u32 = 0;

    while not_edge {
        count += 1;
        let new_height = trees[cy][cx];
        if new_height >= current_height {
            return count;
        }

        if cx == 0 || cx == trees[0].len() - 1 || cy == 0 || cy == trees.len() - 1 {
            not_edge = false;
            break;
        }

        cx = (cx as i32 + direction.0) as usize;
        cy = (cy as i32 + direction.1) as usize;
    }

    return count;
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_08.txt").expect("File read successfully");

    let trees = input.split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let dirs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut best_tree: u32 = 0;

    for y in 1..trees.len() - 1 {
        for x in 1..trees[0].len() - 1 {
            println!("{}, {}", x, y);

            let mut current_tree: u32 = 1;
            for direction in &dirs {
                let res = visible_count(&trees, (x, y), direction);
                println!("{}", res);
                current_tree *= res;
            }

            if current_tree > best_tree {
                best_tree = current_tree;
            }
        }
    }

    // add edges

    println!("{}", best_tree);
}