use crate::utils::file;
use std::collections::HashMap;

const DAY_5_INPUT: &str = "src/input/day05.txt";

pub fn solve() {
    println!("DAY 5 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i32 {
    let (rules, updates) = get_rules_and_updates_from_input();
    // get all rules for before. Check if any before this are in the before after list
    let rule_map = get_rule_map(rules);

    let mut sum = 0;
    for update in updates {
        if is_update_valid(&update, &rule_map) {
            // get the center value and add to the sum
            let center_index = ((update.len() + 1) / 2) - 1;
            sum += update[center_index];
        }
    }

    return sum;
}

fn part2() -> i32 {
    let (rules, updates) = get_rules_and_updates_from_input();
    // get all rules for before. Check if any before this are in the before after list
    let rule_map = get_rule_map(rules);

    let mut sum = 0;
    for update in updates {
        if !is_update_valid(&update, &rule_map) {
            // put the invalid update in the right order
            let valid_update = fix_update(&update, &rule_map);

            // then get the sum in the same way as part 1
            let center_index = ((valid_update.len() + 1) / 2) - 1;
            sum += valid_update[center_index];
        }
    }

    return sum;
}

fn fix_update(update: &Vec<i32>, rule_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut valid_update = update.clone();

    // start right to left
    let mut i = update.len() - 1;
    while i != 0 {
        let page = &valid_update[i];
        // // all of these rules mean that they must come after the page
        let rules = &rule_map[page];

        let mut num_swaps = 0;
        while does_vec_contain_rules(&valid_update[0..i - num_swaps].to_vec(), rules) {
            valid_update.swap(i - num_swaps, i - num_swaps - 1);
            num_swaps += 1;
        }

        if num_swaps == 0 {
            // if all is valid for the current value at this index, then decrement.
            // Otherwise keep going, there should be a different value at i if
            // num_swaps is not 0
            i -= 1;
        }
    }

    return valid_update;
}

fn does_vec_contain_rules(vec: &Vec<i32>, rules: &Vec<i32>) -> bool {
    for rule in rules {
        if vec.contains(rule) {
            return true;
        }
    }
    return false;
}

fn is_update_valid(update: &Vec<i32>, rule_map: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, page) in update.iter().enumerate() {
        // all of these rules mean that they must come after the page
        let rules = &rule_map[page];
        for page_to_compare in &update[0..i] {
            // check if any items before page are in the map vector
            if rules.contains(page_to_compare) {
                return false;
            }
        }
    }
    return true;
}

// gets a map of pages to all the pages that must come after it
fn get_rule_map(rules: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut rule_map = HashMap::new();

    for rule in rules {
        rule_map
            .entry(rule.0)
            .and_modify(|vec: &mut Vec<i32>| vec.extend(vec![rule.1]))
            .or_insert(vec![rule.1]);
    }

    return rule_map;
}

fn get_rules_and_updates_from_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let lines = match file::read_lines_from_text(DAY_5_INPUT) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut processed_rules = false;
    for line in lines {
        if line.is_empty() {
            processed_rules = true;
            continue;
        }

        if processed_rules {
            let parts: Vec<&str> = line.split(",").collect();
            let mut update = Vec::new();
            for part in parts {
                let parsed = part
                    .parse::<i32>()
                    .expect("Error parsing number for update");
                update.push(parsed);
            }
            updates.push(update);
        } else {
            let parts: Vec<&str> = line.split("|").collect();
            if parts.len() != 2 {
                panic!("Expected two numbers, but got: {}", line);
            }
            let first = parts[0].parse::<i32>().expect("Error parsing first number");
            let second = parts[1]
                .parse::<i32>()
                .expect("Error parsing second number");
            rules.push((first, second));
        }
    }

    return (rules, updates);
}
