use std::fs;
use std::ops::Deref;

fn substring_ascii<'a>(input: &'a str, start: &str, end: &str) -> &'a str {
    let start_idx = input.find(start).unwrap_or(0);
    let end_idx = input.find(end).unwrap_or(input.len());

    return &input[(start_idx + 1)..end_idx];
}

fn get_sublist(input: &Vec<String>, start: usize) -> String {
    let mut bracket_count = 0;
    let mut res: String = String::new();

    for i in start..input.len() {
        res.push_str(&*input[i]);
        res.push_str(",");

        bracket_count += input[i].chars().filter(|ele| *ele == '[').count();
        bracket_count -= input[i].chars().filter(|ele| *ele == ']').count();

        if bracket_count == 0 {
            return (&res[1..res.len() - 2]).to_string();
        }
    }

    assert!(false);
    return String::new();
}

fn is_right_order(packets_left: &str, packets_right: &str) -> Option<bool> {
    let left_list_tmp = packets_left.split(",").filter(|ele| ele.len() != 0).map(|ele| ele.to_string()).collect::<Vec<String>>();
    let right_list_tmp = packets_right.split(",").filter(|ele| ele.len() != 0).map(|ele| ele.to_string()).collect::<Vec<String>>();

    let mut left_list: Vec<String> = Vec::new();
    let mut skip = 0;
    for i in 0..left_list_tmp.len() {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        if left_list_tmp[i].contains('[') {
            let tmp = get_sublist(&left_list_tmp, i);
            skip = tmp.chars().filter(|ele| *ele == ',').count();
            let mut s = String::new();
            s.push('[');
            s.push_str(&*tmp);
            s.push(']');

            left_list.push(s);
            continue;
        }

        left_list.push(left_list_tmp[i].to_string());
    }

    let mut right_list: Vec<String> = Vec::new();
    let mut skip = 0;
    for i in 0..right_list_tmp.len() {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        if right_list_tmp[i].contains('[') {
            let tmp = get_sublist(&right_list_tmp, i);
            skip = tmp.chars().filter(|ele| *ele == ',').count();
            let mut s = String::new();
            s.push('[');
            s.push_str(&*tmp);
            s.push(']');

            right_list.push(s);
            continue;
        }

        right_list.push(right_list_tmp[i].to_string());
    }

    let mut skip_after_recursion = false;
    let mut skip_after_recursion_right = false;

    // println!("{} {}", left_list.len(), right_list.len());
    for i in 0..left_list.len() {
        if i >= right_list.len() {
            println!("False based on len");
            return Some(false);
        }

        let left_item = &left_list[i];
        let right_item = &right_list[i];

        if left_item.starts_with("[") {
            let left_list_new = &left_item[1..left_item.len() - 1].to_string();

            let right_list_new: String = if right_item.starts_with("[") {
                right_item[1..right_item.len() - 1].to_string()
            } else { right_item.to_string() };

            let res = is_right_order(&*left_list_new, &*right_list_new);
            if res.is_some() {
                return res;
            }
            continue;
        }

        if right_item.starts_with("[") {
            let right_list_new = &right_item[1..right_item.len() - 1].to_string();

            let left_list_new: String = if left_item.starts_with("[") {
                left_item[1..left_item.len() - 1].to_string()
            } else { left_item.to_string() };

            let res = is_right_order(&*left_list_new, &*right_list_new);
            if res.is_some() {
                return res;
            }
            continue;
        }

        let left_item_num = left_item.parse::<u32>().unwrap();
        let right_item_num = right_item.parse::<u32>().unwrap();

        if right_item_num > left_item_num {
            return Some(true);
        } else if left_item_num > right_item_num {
            return Some(false);
        }
    }

    if right_list.len() == left_list.len() {
        return None;
    }

    return Some(true);
}

#[allow(dead_code)]
pub fn solve() {
    let input: String = fs::read_to_string("inputs/input_13.txt").expect("File read successfully");

    let mut packages: Vec<&str> = input.split("\n").filter(|ele| ele.len() != 0).collect::<Vec<&str>>();
    packages.push("[[2]]");
    packages.push("[[6]]");

    let mut sorted = false;
    let mut res = 0;

    let mut order = (0..packages.len()).collect::<Vec<usize>>();
    while !sorted {
        sorted = true;
        let mut new_order = order.clone();
        let mut prev: usize = packages.len();
        let mut i = 1;
        for ele in order {
            if prev == packages.len() {
                prev = ele;
                continue;
            }

            println!("{} \n {} \n ---------------", packages[prev], packages[ele]);
            let mut ordered = is_right_order(packages[prev], packages[ele]).unwrap();
            if !ordered {
                sorted = false;
                new_order.swap(i, i - 1);
            }
            i += 1;
            prev = ele;
        }
        order = new_order;
    }

    let mut found_2: usize = 0;
    let mut found_6: usize = 0;

    let mut i = 0;
    for ele in order {
        if ele == packages.len() - 1 {
            found_6 = i + 1;
        }

        if ele == packages.len() - 2 {
            found_2 = i + 1;
        }

        i += 1;
    }
    println!("{}", found_2 * found_6);
}