use std::collections::HashMap;
use crate::utils::file;

const DAY_11_INPUT: &str = "src/input/day11.txt";

pub fn solve() {
    println!("DAY 11 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i64 {
    let stones = get_stones_from_input();
    let mut stone_map = get_stone_map(stones);

    for _i in 0..25 {
        stone_map = blink_map(stone_map);
    }

    return stone_map.values().sum::<i64>();
}

fn part2() -> i64 {
    let stones = get_stones_from_input();
    let mut stone_map = get_stone_map(stones);

    for _i in 0..75 {
        stone_map = blink_map(stone_map);
    }

    return stone_map.values().sum::<i64>();
}

fn get_stone_map(stones: Vec<i64>) -> HashMap<i64, i64> {
    let mut map = HashMap::new();
    for stone in stones {
        map.entry(stone)
            .and_modify(|existing_value| *existing_value += 1) 
            .or_insert(1);
    }
    return map;
}

fn blink_map(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_stones = HashMap::new();
    for (k, v) in stones.iter() {
        let blink_res = blink_one_stone(k);
        for stone in blink_res {
            new_stones.entry(stone)
                .and_modify(|existing_value| *existing_value += v) 
                .or_insert(*v);
        }
    }

    return new_stones;
}
fn blink_one_stone(stone: &i64) -> Vec<i64> {
    let mut new_stones = Vec::new();
    if stone == &0 {
        new_stones.push(1);
        return new_stones;
    }

    if has_even_number_of_digits(stone) {
        let (first, second) = split_stone(stone);
        new_stones.push(first);
        new_stones.push(second);
        return new_stones;
    }

    new_stones.push(stone * 2024);
    return new_stones;
}

fn split_stone(n: &i64) -> (i64, i64) {
    let num_str = n.to_string();
    let mid = num_str.len() / 2;
    let (first_half_str, second_half_str) = num_str.split_at(mid);
    let first_half = first_half_str.parse::<i64>().unwrap();
    let second_half = second_half_str.parse::<i64>().unwrap();
    return (first_half, second_half);
}

fn has_even_number_of_digits(n: &i64) -> bool {
    let num_as_string = n.to_string();
    return num_as_string.len() % 2 == 0
}

fn get_stones_from_input() -> Vec<i64> {
    let lines = match file::read_lines_from_text(DAY_11_INPUT) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    if lines.len() != 1 {
        panic!("only 1 line expected");
    }

    let mut stones = Vec::new();
    let stone_strings: Vec<&str> = lines[0].split_whitespace().collect();

    for ss in stone_strings {
        let new_stone = ss
            .parse::<i64>()
            .map_err(|e| format!("Error parsing stone string: {}", e)).unwrap();
        stones.push(new_stone);
    }

    return stones;
}