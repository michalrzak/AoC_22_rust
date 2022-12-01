use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let input = fs::read_to_string("inputs/input_1.txt").expect("File was successfully read");
    let elves = input.split("\n\n").map(|calories| calories.split('\n')
        .map(|calory| calory.parse::<i32>().expect("Successfully converted calory to integer")).sum::<i32>()).collect::<Vec<i32>>();

    let mut ranking:Vec<i32> = vec![0, 0, 0];
    ranking.iter().for_each(|x| println!("{}", x));

    for elf in elves {
        for i in 0..ranking.len() {
            if elf > ranking[i] {
                for j in i+1..ranking.len() {
                    ranking[j] = ranking[j-1];
                }
                ranking[i] = elf;
                break;
            }
        }

    }

    let result:i32 = ranking.iter().sum();
    ranking.iter().for_each(|x| println!("{}", x));
    println!("{}", result);
}