use std::collections::HashMap;
use std::fs;

struct Monkey {
    items: Vec<u32>,
    multiplication: bool,
    operation_number: Option<u32>,
    test_number: u32,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn operation(&self, old: u32) -> u32 {
        let other = self.operation_number.unwrap_or(old);

        if self.multiplication {
            return old * other;
        }

        return old + other;
    }

    fn test(&self, val: u32) -> usize {
        if val % self.test_number == 0 {
            return self.true_monkey;
        }

        return self.false_monkey;
    }
}


#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_11.txt").expect("File read successfully");

    let monkey_strings = input.split("\n\n");
    let mut monkeys: Vec<Monkey> = monkey_strings.map(|monkey_string| {
        let lines = monkey_string.split("\n").collect::<Vec<&str>>();
        let items = lines[1].split(": ").skip(1).next().unwrap()
            .split(", ").map(|ele| ele.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let operation_information = lines[2].split(": ").skip(1).next().unwrap()
            .split(" ").collect::<Vec<&str>>();
        let operation_number: Option<u32> = operation_information.last().unwrap().parse::<u32>().ok();
        let multiplication: bool =operation_information[3] == "*";

        // /*
        //     This once again does not work and I don't know any more how to fix it. the issue is that the `number`
        //     variable gets discarded when out it gets out of scope
        //  */
        // let operation: Box::<dyn Fn(u32) -> u32> = if operation_information[3] == "+" {
        //     Box::new(|old| old + number)
        // } else {
        //     Box::new(|old| old * number)
        // };

        let test_number: u32 = lines[3].split(" ").last().unwrap().parse::<u32>().unwrap();
        let true_monkey: usize = lines[4].split(" ").last().unwrap().parse::<usize>().unwrap();
        let false_monkey: usize = lines[5].split(" ").last().unwrap().parse::<usize>().unwrap();

        //let test: Box::<dyn Fn(u32) -> usize> = Box::new(|item| if item % test_number == 0 { true_monkey } else { false_monkey });

        return Monkey { items, multiplication, operation_number, test_number, true_monkey, false_monkey };
    }).collect::<Vec<Monkey>>();

    println!("Monkey count: {}", monkeys.len());

    let mut new_items: Vec<Vec<u32>> = Vec::new();
    let mut monkey_inspect: Vec<u32> = Vec::new();
    for i in 0..monkeys.len() {
        new_items.insert(i, Vec::new());
        monkey_inspect.push(0);
    }

    for i in 0..20 {
        let mut monkey_count: usize = 0;
        for monkey in &mut monkeys {
            for item in &new_items[monkey_count] {
                monkey.items.push(*item);
            }
            let _ = std::mem::replace(&mut new_items[monkey_count], Vec::new());

            for item in &monkey.items {
                let old_inspect = monkey_inspect[monkey_count];
                let _ = std::mem::replace(&mut monkey_inspect[monkey_count], old_inspect + 1);

                let new_item = monkey.operation(*item);
                let bored_item = new_item / 3;
                let new_monkey = monkey.test(bored_item);
                new_items.get_mut(new_monkey).unwrap().push(bored_item);
            }

            monkey.items = Vec::new();

            monkey_count += 1;
        }

    }

    for monkey in &monkey_inspect {
        println!("{}", monkey);
    }

    monkey_inspect.sort();

    println!("{}", monkey_inspect[monkey_inspect.len() - 1] * monkey_inspect[monkey_inspect.len() - 2]);


}