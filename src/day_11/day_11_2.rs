/*
    BigUint is technically unnecessary, but I left it in as the program still ran fine
 */
use std::collections::HashMap;
use std::fs;

use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, One};

struct Monkey {
    items: Vec<BigUint>,
    multiplication: bool,
    operation_number: Option<BigUint>,
    test_number: BigUint,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn operation(&self, old: &BigUint) -> BigUint {
        let other = self.operation_number.as_ref().unwrap_or(old);

        let mut ret = BigUint::from(0u32);

        if self.multiplication {
            ret = old * other;
        } else {
            ret = old + other;
        }

        let a:BigUint = BigUint::from(11u32 * 19u32 * 5u32 * 2u32 * 13u32 * 7u32 * 3u32 * 17u32);
        println!("{} vs. {}", ret, a);
        if ret >= a {
            ret %= a;
        }

        return ret;
    }

    fn test(&self, val: &BigUint) -> usize {
        if val % &self.test_number ==  Zero::zero() {
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
            .split(", ").map(|ele| ele.parse::<BigUint>().unwrap()).collect::<Vec<BigUint>>();

        let operation_information = lines[2].split(": ").skip(1).next().unwrap()
            .split(" ").collect::<Vec<&str>>();
        let operation_number: Option<BigUint> = operation_information.last().unwrap().parse::<BigUint>().ok();
        let multiplication: bool =operation_information[3] == "*";

        // /*
        //     This once again does not work and I don't know any more how to fix it. the issue is that the `number`
        //     variable gets discarded when out it gets out of scope
        //  */
        // let operation: Box::<dyn Fn(BigUint) -> BigUint> = if operation_information[3] == "+" {
        //     Box::new(|old| old + number)
        // } else {
        //     Box::new(|old| old * number)
        // };

        let test_number: BigUint = lines[3].split(" ").last().unwrap().parse::<BigUint>().unwrap();
        let true_monkey: usize = lines[4].split(" ").last().unwrap().parse::<usize>().unwrap();
        let false_monkey: usize = lines[5].split(" ").last().unwrap().parse::<usize>().unwrap();

        //let test: Box::<dyn Fn(BigUint) -> usize> = Box::new(|item| if item % test_number == 0 { true_monkey } else { false_monkey });

        return Monkey { items, multiplication, operation_number, test_number, true_monkey, false_monkey };
    }).collect::<Vec<Monkey>>();

    println!("Monkey count: {}", monkeys.len());

    let mut new_items: Vec<Vec<BigUint>> = Vec::new();
    let mut monkey_inspect: Vec<u32> = Vec::new();
    for i in 0..monkeys.len() {
        new_items.insert(i, Vec::new());
        monkey_inspect.push(Zero::zero());
    }

    for i in 0..10000 {
        println!("{}", i);
        let mut monkey_count: usize = 0;
        for monkey in &mut monkeys {
            for item in &new_items[monkey_count] {
                monkey.items.push(item.clone());
            }
            let _ = std::mem::replace(&mut new_items[monkey_count], Vec::new());

            for item in &monkey.items {
                let old_inspect: u32 = monkey_inspect[monkey_count];
                let _ = std::mem::replace(&mut monkey_inspect[monkey_count], old_inspect + 1);

                let new_item = monkey.operation(item);
                let new_monkey = monkey.test(&new_item);
                new_items.get_mut(new_monkey).unwrap().push(new_item);
            }

            monkey.items = Vec::new();

            monkey_count += 1;
        }

    }

    for monkey in &monkey_inspect {
        println!("{}", monkey);
    }

    monkey_inspect.sort();

    println!("{}", monkey_inspect[monkey_inspect.len() - 1] as u64 * monkey_inspect[monkey_inspect.len() - 2] as u64);


}