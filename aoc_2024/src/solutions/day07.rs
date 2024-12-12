use crate::utils::file;

const DAY_7_INPUT: &str = "src/input/day07.txt";

pub fn solve() {
    println!("DAY 7 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i64 {
    let equations = get_equations_from_input();

    let mut equations_total_sum = 0;
    for (total, numbers) in equations {
        if does_equation_compute(&total, &numbers[0], &numbers[1..]) {
            equations_total_sum += total
        }
    }

    return equations_total_sum;
}

fn part2() -> i64 {
    let equations = get_equations_from_input();

    let mut equations_total_sum = 0;
    for (total, numbers) in equations {
        if does_equation_compute_with_combine(&total, &numbers[0], &numbers[1..]) {
            equations_total_sum += total
        }
    }

    return equations_total_sum;
}

fn does_equation_compute(total: &i64, current_res: &i64, numbers: &[i64]) -> bool {
    if numbers.len() == 0 && current_res == total {
        return true;
    }

    if numbers.len() == 0 {
        return false;
    }

    let plus_res = does_equation_compute(total, &(current_res + numbers[0]), &numbers[1..]);

    let mult_res = does_equation_compute(total, &(current_res * numbers[0]), &numbers[1..]);

    return plus_res || mult_res;
}

fn does_equation_compute_with_combine(total: &i64, current_res: &i64, numbers: &[i64]) -> bool {
    if numbers.len() == 0 && current_res == total {
        return true;
    }

    if numbers.len() == 0 {
        return false;
    }

    let plus_res =
        does_equation_compute_with_combine(total, &(current_res + numbers[0]), &numbers[1..]);

    let mult_res =
        does_equation_compute_with_combine(total, &(current_res * numbers[0]), &numbers[1..]);

    let combine_res = does_equation_compute_with_combine(
        total,
        &(combine(*current_res, numbers[0])),
        &numbers[1..],
    );

    return plus_res || mult_res || combine_res;
}

fn combine(first: i64, second: i64) -> i64 {
    // Convert both numbers to strings and concatenate them
    let combined_string = format!("{}{}", first, second);

    // Parse the concatenated string back to an i64
    let combined_number: i64 = combined_string
        .parse()
        .expect("Failed to parse combined number");

    return combined_number;
}

fn get_equations_from_input() -> Vec<(i64, Vec<i64>)> {
    let lines = match file::read_lines_from_text(DAY_7_INPUT) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut equations = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 2 {
            panic!("Expected two parts, but got: {}", line);
        }

        let total = parts[0]
            .parse::<i64>()
            .expect("Error parsing number for update");

        let numbers: Vec<&str> = parts[1].trim().split_whitespace().collect();
        let mut parsed_numbers = Vec::new();
        for num in numbers {
            let parsed_number = num.parse::<i64>().expect("Error parsing number for update");
            parsed_numbers.push(parsed_number);
        }

        equations.push((total, parsed_numbers));
    }

    return equations;
}
