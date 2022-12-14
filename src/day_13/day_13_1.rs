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

    let package_pairs: Vec<(&str, &str)> = input.split("\n\n").map(|pair| {
        let v = pair.split("\n").collect::<Vec<&str>>();
        (v[0], v[1])
    }).collect::<Vec<(&str, &str)>>();

    let mut res = 0;
    let mut i = 1;
    for ele in package_pairs {
        let tmp_res = is_right_order(ele.0, ele.1).unwrap();
        println!("{}", tmp_res);
        if tmp_res {
            res += i;
        }
        i += 1;
    }

    println!("{}", res);
}