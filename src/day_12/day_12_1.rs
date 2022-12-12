use std::collections::{HashMap, HashSet};
use std::fs;

#[allow(dead_code)]
fn find_start(height_map: &Vec<Vec<u32>>) -> (usize, usize) {
    for y in 0 .. height_map.len() {
        for x in 0 .. height_map[0].len() {
            if height_map[y][x] == 50 {
                return (x, y);
            }
        }
    }
    assert!(false);
    (0, 0)
}

#[allow(dead_code)]
fn find_end(height_map: &Vec<Vec<u32>>) -> (usize, usize) {
    for y in 0 .. height_map.len() {
        for x in 0 .. height_map[0].len() {
            if height_map[y][x] == 'z' as u32 - 'a' as u32 {
                return (x, y);
            }
        }
    }
    assert!(false);
    (0, 0)
}

#[allow(dead_code)]
fn get_smallest(distances: &HashMap<(usize, usize), u32>,
                unvisited: &HashSet<(usize, usize)>,
                known: &HashSet<(usize, usize)>) -> (usize, usize){
    let mut smallest = 0u32;
    let mut found = false;
    let mut smallest_pos = (0usize, 0usize);

    for pos in unvisited {
        if known.contains(pos) {
            println!("[{}, {}]", pos.0, pos.1);
            if !found {
                smallest = distances[pos];
                smallest_pos = *pos;
                found = true;
                continue;
            }

            if smallest > distances[pos] {
                smallest = distances[pos];
                smallest_pos = *pos;
            }
        }
    }

    assert!(found);
    return smallest_pos;
}

#[allow(dead_code)]
fn dijkstra(height_map: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let height = height_map.len();
    let width = height_map[0].len();

    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();

    let mut known: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    for y in 0 .. height {
        for x in 0 .. width {
            unvisited.insert((x, y));
            distances.insert((x, y), 0);
        }
    }


    let mut current_node = start;
    known.insert(start);

    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    while current_node != end {
        println!("{}, {}", current_node.0, current_node.1);

        for dir in &dirs {
            let new_x = (current_node.0 as i32 + dir.0 + 1) as usize;
            let new_y = (current_node.1 as i32 + dir.1 + 1) as usize;
            if new_x > width || new_x <= 0 || new_y > height || new_y <= 0 {
                continue;
            }
            let new: (usize, usize) = (new_x - 1, new_y - 1);

            let from = height_map[current_node.1][current_node.0];
            let to = height_map[new.1][new.0];
            if to >= from + 2 {
                continue;
            }

            let new_distance: u32 = distances[&current_node] + 1;
            if !known.contains(&new) || new_distance < distances[&new] {
                let _ = std::mem::replace(distances.get_mut(&new).unwrap(), new_distance);
                known.insert(new);
            }
        }

        unvisited.remove(&current_node);
        current_node = get_smallest(&distances, &unvisited, &known);
    }

    return distances[&current_node] + 1;
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_12.txt").expect("File read successfully");
    let height_map: Vec<Vec<u32>> = input.split("\n")
        .map(|line|
            line.chars().map(|c| {
                if c == 'S' { return 50; }
                if c == 'E' { return 'z' as u32 - 'a' as u32 + 1; }
                c as u32 - 'a' as u32
            }).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let start_pos: (usize, usize) = find_start(&height_map);
    let end_pos: (usize, usize) = find_end(&height_map);

    let res = dijkstra(&height_map, start_pos, end_pos);
    println!("{}", res);
}