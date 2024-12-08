use crate::utils::file;
use regex::Regex;

const DAY_3_INPUT: &str = "src/input/day03.txt";

pub fn solve() {
    println!("DAY 3 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i32 {
    let instructions = get_instructions_from_input();
    let mut sum = 0;
    for instr in instructions {
        match instr.as_str() {
            "do()" => continue,
            "don't()" => continue,
            _ => {
                let (x, y) = get_multiply_values_from_string(&instr);
                sum += x * y
            }
        }
    }
    return sum;
}

fn part2() -> i32 {
    let mut enabled = true;

    let mut sum = 0;
    let instructions = get_instructions_from_input();
    for instr in instructions {
        match instr.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let (x, y) = get_multiply_values_from_string(&instr);
                    sum += x * y
                }
            }
        }
    }

    return sum;
}

fn get_instructions_from_input() -> Vec<String> {
    let lines = match file::read_lines_from_text(DAY_3_INPUT) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut all_instr = Vec::new();

    for line in lines {
        let instr = get_instructions_from_string(&line);
        all_instr.extend(instr);
    }

    return all_instr;
}

fn get_instructions_from_string(s: &String) -> Vec<String> {
    // regex for multiplication, do, and don't instructions
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut instructions = Vec::new();
    for cap in re.find_iter(s) {
        instructions.push(cap.as_str().to_string())
    }

    return instructions;
}

fn get_multiply_values_from_string(s: &String) -> (i32, i32) {
    // Define the regular expression
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut caps = re.captures_iter(s);

    let cap = caps.next().unwrap();

    if caps.count() > 1 {
        panic!("multiple multiply values found in supplied string {}", s)
    }

    if let (Ok(x), Ok(y)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
        return (x, y);
    } else {
        panic!(
            "failed to parse ints from string: {}",
            cap.get(0).unwrap().as_str()
        )
    }
}
