use crate::utils::file;

const DAY_2_INPUT: &str = "src/input/day02.txt";

pub fn solve() {
    println!("DAY 2 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i32 {
    let reports = read_reports_from_input();

    let mut num_safe_reports = 0;
    for report in reports {
        if is_report_safe(&report) {
            num_safe_reports += 1;
        }
    }
    return num_safe_reports;
}

// in part 2 a report is considered safe if removing 1 level from the
// report results in a safe report under the same rules as part 1
fn part2() -> i32 {
    let reports = read_reports_from_input();

    let mut num_safe_reports = 0;
    for report in reports {
        // try the entire report
        if is_report_safe(&report) {
            num_safe_reports += 1;
            continue;
        }

        for i in 0..report.len() {
            // remove each level from the report
            let mut new_report = report.clone();
            new_report.remove(i);
            if is_report_safe(&new_report) {
                num_safe_reports += 1;
                break;
            }
        }
    }
    return num_safe_reports;
}

// A report is considered safe if both of the following are true:
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut increasing = false;

    for (i, level) in report.iter().enumerate() {
        // can't compare the first value to the last value
        if i == 0 {
            continue;
        }

        // convenience
        let last_level = report[i - 1];

        // set increasing if on second level
        if i == 1 && *level > last_level {
            increasing = true
        }

        // has to increase/decrease by at least one
        if *level == last_level {
            return false;
        }

        // cannot increase/decrease by more than three
        if (level - last_level).abs() > 3 {
            return false;
        }

        // if increasing and current value is less than last value
        if increasing && *level < last_level {
            return false;
        }

        // if decreasing and current value is greater than last value
        if !increasing && *level > last_level {
            return false;
        }
    }

    return true;
}

fn read_reports_from_input() -> Vec<Vec<i32>> {
    let lines = match file::read_lines_from_text(DAY_2_INPUT) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut reports = Vec::new();
    for line in lines {
        let report = match parse_line(&line) {
            Ok(report) => report,
            Err(e) => {
                panic!("Error parsing line: {}", e);
            }
        };
        reports.push(report);
    }
    return reports;
}

fn parse_line(line: &str) -> Result<Vec<i32>, String> {
    let levels: Vec<&str> = line.split_whitespace().collect();

    let mut report = Vec::new();
    for l in levels {
        let new_level = l
            .parse::<i32>()
            .map_err(|e| format!("Error parsing level: {}", e))?;
        report.push(new_level);
    }

    Ok(report)
}
