use std::collections::HashSet;
use crate::utils::{direction, file};

const DAY_10_INPUT: &str = "src/input/day10.txt";

pub fn solve() {
    println!("DAY 10 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i32 {
    let matrix = file::get_i32_matrix_from_text_input(DAY_10_INPUT);
    let trailheads = get_trailheads(&matrix);

    let mut sum = 0;
    for t in trailheads {
        sum += get_trailhead_score(t, &matrix);
    }

    return sum;
}

fn part2() -> i32 {
    let matrix = file::get_i32_matrix_from_text_input(DAY_10_INPUT);
    let trailheads = get_trailheads(&matrix);

    let mut sum = 0;
    for t in trailheads {
        sum += get_trailhead_rating(t, &matrix);
    }

    return sum;
}

fn get_trailhead_rating(trailhead: (usize, usize), matrix: &Vec<Vec<i32>>) -> i32 {
    let visited = Vec::new();
    let max_y = matrix.len() - 1;
    let max_x = matrix[0].len() - 1;
    match find_9_trails(trailhead, &visited, &0, matrix, &max_x, &max_y, &Vec::new()) {
        Ok(res) => {
            return res.len() as i32;
        },
        Err(_e) => {
            return 0;
        }
    }
}

fn find_9_trails(
    pos: (usize, usize),
    visited: &Vec<(usize,usize)>,
    cur_top: &i32,
    matrix: &Vec<Vec<i32>>,
    max_x: &usize,
    max_y: &usize,
    found: &Vec<(usize, usize)>
) -> Result<Vec<(usize, usize)>, String> {
    if visited.contains(&(pos.0, pos.1)) {
        return Err("already visited".to_string());
    }

    let mut new_visited = visited.clone();
    new_visited.push((pos.0, pos.1));

    if cur_top == &9 {
        let mut new_found = found.clone();
        new_found.push((pos.0, pos.1));
        return Ok(new_found);
    }
    
    let up_results: Vec<(usize, usize)> = match find_9_trails_in_dir(direction::Direction::Up, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let right_results = match find_9_trails_in_dir(direction::Direction::Right, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let down_results = match find_9_trails_in_dir(direction::Direction::Down, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let left_results = match find_9_trails_in_dir(direction::Direction::Left, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let mut combined_trails: Vec<(usize, usize)> = Vec::new();
    combined_trails.extend(found);
    combined_trails.extend(up_results);
    combined_trails.extend(right_results);
    combined_trails.extend(down_results);
    combined_trails.extend(left_results);
    

    return Ok(combined_trails);
}

fn get_trailhead_score(trailhead: (usize, usize), matrix: &Vec<Vec<i32>>) -> i32 {
    let visited = Vec::new();
    let max_y = matrix.len() - 1;
    let max_x = matrix[0].len() - 1;
    match find_9_heights(trailhead, &visited, &0, matrix, &max_x, &max_y, &Vec::new()) {
        Ok(res) => {
            return res.len() as i32;
        },
        Err(_e) => {
            return 0;
        }
    }
}

fn find_9_heights(
    pos: (usize, usize),
    visited: &Vec<(usize,usize)>,
    cur_top: &i32,
    matrix: &Vec<Vec<i32>>,
    max_x: &usize,
    max_y: &usize,
    found: &Vec<(usize, usize)>
) -> Result<Vec<(usize, usize)>, String> {
    if visited.contains(&(pos.0, pos.1)) {
        return Err("already visited".to_string());
    }

    let mut new_visited = visited.clone();
    new_visited.push((pos.0, pos.1));

    if cur_top == &9 {
        let mut new_found = found.clone();
        new_found.push((pos.0, pos.1));
        return Ok(new_found);
    }
    
    let up_results: Vec<(usize, usize)> = match find_9_heights_in_dir(direction::Direction::Up, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let right_results = match find_9_heights_in_dir(direction::Direction::Right, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let down_results = match find_9_heights_in_dir(direction::Direction::Down, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let left_results = match find_9_heights_in_dir(direction::Direction::Left, pos, &new_visited, cur_top, matrix, max_x, max_y, found) {
        Ok(res) => res,
        Err(_e) => {Vec::new()}
    };

    let mut combined_set: HashSet<(usize, usize)> = HashSet::new();
    combined_set.extend(found);
    combined_set.extend(up_results);
    combined_set.extend(right_results);
    combined_set.extend(down_results);
    combined_set.extend(left_results);
    

    return Ok(combined_set.into_iter().collect());


}

fn find_9_trails_in_dir(
    d: direction::Direction,
    pos: (usize, usize),
    visited: &Vec<(usize,usize)>,
    cur_top: &i32,
    matrix: &Vec<Vec<i32>>,
    max_x: &usize,
    max_y: &usize,
    found: &Vec<(usize, usize)>
) -> Result<Vec<(usize, usize)>, String> {
    let (next_x, next_y) = match d.get_next_pos(&pos.0, &pos.1, max_x, max_y) {
        Ok((x, y)) => {
            if matrix[y][x] != cur_top + 1 {
                return Err("next position must be cur + 1".to_string());
            }
            (x, y)
        },
        Err(e) => {
            return Err(e);
        }
    };

    return find_9_trails((next_x, next_y), visited, &(cur_top + 1), matrix, max_x, max_y, found);
}

fn find_9_heights_in_dir(
    d: direction::Direction,
    pos: (usize, usize),
    visited: &Vec<(usize,usize)>,
    cur_top: &i32,
    matrix: &Vec<Vec<i32>>,
    max_x: &usize,
    max_y: &usize,
    found: &Vec<(usize, usize)>
) -> Result<Vec<(usize, usize)>, String> {
    let (next_x, next_y) = match d.get_next_pos(&pos.0, &pos.1, max_x, max_y) {
        Ok((x, y)) => {
            if matrix[y][x] != cur_top + 1 {
                return Err("next position must be cur + 1".to_string());
            }
            (x, y)
        },
        Err(e) => {
            return Err(e);
        }
    };

    return find_9_heights((next_x, next_y), visited, &(cur_top + 1), matrix, max_x, max_y, found);
}

fn get_trailheads(matrix: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if val == &0 {
                trailheads.push((x, y));
            }
        }
    }
    return trailheads;
}