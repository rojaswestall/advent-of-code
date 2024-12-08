use std::collections::HashMap;

use crate::utils::file;

const DAY_1_INPUT: &str = "src/input/day01.txt";

pub fn solve() {
    println!("DAY 1 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i32 {
    let (mut left_list, mut right_list) = read_lists_from_input();

    left_list.sort();
    right_list.sort();

    let mut total_diff = 0;
    for i in 0..left_list.len() {
        let diff = (left_list[i] - right_list[i]).abs();
        total_diff += diff
    }

    return total_diff;
}

fn part2() -> i32 {
    let (left_list, right_list) = read_lists_from_input();

    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score = 0;

    for i in 0..left_list.len() {
        // if it's already been calculated use the calculated amount
        if let Some(&frequency) = occurrences.get(&left_list[i]) {
            similarity_score += left_list[i] * &frequency;
            continue;
        }

        let frequency = find_occurrences_in_list(left_list[i], &right_list);
        occurrences.insert(left_list[i], frequency);
        similarity_score += left_list[i] * &frequency;
    }

    return similarity_score;
}

fn find_occurrences_in_list(item: i32, list: &Vec<i32>) -> i32 {
    let mut frequency = 0;
    for l in list {
        if item == *l {
            frequency += 1;
        }
    }
    return frequency;
}

fn read_lists_from_input() -> (Vec<i32>, Vec<i32>) {
    let lines = match file::read_lines_from_text(DAY_1_INPUT) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in lines {
        let (left, right) = match parse_line(&line) {
            Ok(nums) => nums,
            Err(e) => {
                panic!("Error parsing line: {}", e);
            }
        };
        left_list.push(left);
        right_list.push(right);
    }

    return (left_list, right_list);
}

fn parse_line(line: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 2 {
        return Err(format!("Expected two numbers, but got: {}", line));
    }

    let first = parts[0]
        .parse::<i32>()
        .map_err(|e| format!("Error parsing first number: {}", e))?;
    let second = parts[1]
        .parse::<i32>()
        .map_err(|e| format!("Error parsing second number: {}", e))?;

    Ok((first, second))
}
