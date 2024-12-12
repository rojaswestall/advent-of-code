use std::{collections::HashSet, fmt};

use crate::utils::file;

const DAY_6_INPUT: &str = "src/input/day06.txt";
const OPEN: &str = ".";
const OBSTACLE: &str = "#";

pub fn solve() {
    println!("DAY 6 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i32 {
    let matrix = file::get_string_matrix_from_text_input(DAY_6_INPUT);
    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;

    // I cheated and looked at the input. You could write some helper
    // methods to determine the direction and not just the location
    let mut cur_dir = Direction::Up;
    let (mut x, mut y) = find_start(&matrix);

    let mut visited = Vec::new();

    loop {
        // keep track of visited coordinates
        if !visited.contains(&(x, y)) {
            visited.push((x, y));
        }

        // get next position, if next position is out of bounds exit
        let (next_x, next_y) = match get_next_pos(&cur_dir, &x, &y, &max_x, &max_y) {
            Ok((x, y)) => (x, y),
            Err(_e) => {
                // next position is out of bounds
                break;
            }
        };

        if matrix[next_y][next_x] == OBSTACLE {
            // instead of moving to the next space, we just change direction
            cur_dir = get_next_direction(&cur_dir);
            continue;
        }

        // move onto the next position
        x = next_x;
        y = next_y;
    }

    return visited.len().try_into().unwrap();
}

fn part2() -> i32 {
    let mut matrix = file::get_string_matrix_from_text_input(DAY_6_INPUT);
    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;

    // I cheated and looked at the input. You could write some helper
    // methods to determine the direction and not just the location
    let mut cur_dir = Direction::Up;
    let (mut x, mut y) = find_start(&matrix);

    // we want to know which positions will create loops, so we also
    // need to keep track of the direction we were going when we visited
    // a specific position
    let mut visited = HashSet::new();
    let mut loop_obstacles = Vec::new();

    loop {
        visited.insert((x, y));

        // get next position, if next position is out of bounds exit
        let (next_x, next_y) = match get_next_pos(&cur_dir, &x, &y, &max_x, &max_y) {
            Ok((x, y)) => (x, y),
            Err(_e) => {
                // next position is out of bounds
                break;
            }
        };

        if matrix[next_y][next_x] == OBSTACLE {
            // instead of moving to the next space, we just change direction.
            // placing a new obstacle where an existing obstacle already existed
            // wouldn't create a loop so we don't need to worry about this
            cur_dir = get_next_direction(&cur_dir);
            continue;
        }

        if !visited.contains(&(next_x, next_y)) {
            matrix[next_y][next_x] = OBSTACLE.to_string();
            if is_loop(&cur_dir, x, y, &matrix) {
                // if there were an obstacle at the next position
                // there would be a loop, add next position to loops
                loop_obstacles.push((next_x, next_y))
            }
            matrix[next_y][next_x] = OPEN.to_string();
        }

        // move onto the next position
        x = next_x;
        y = next_y;
    }

    return loop_obstacles.len().try_into().unwrap();
}

fn is_loop(d: &Direction, x: usize, y: usize, matrix: &Vec<Vec<String>>) -> bool {
    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;
    let mut cur_dir = *d;
    let mut cur_x = x;
    let mut cur_y = y;

    // we want to know which positions will create loops, so we also
    // need to keep track of the direction we were going when we visited
    // a specific position. It's a loop if we see the same visited obstacle
    let mut visited_obstacles: Vec<(usize, usize, Direction)> = Vec::new();

    loop {
        // get next position, if next position is out of bounds exit
        let (next_x, next_y) = match get_next_pos(&cur_dir, &cur_x, &cur_y, &max_x, &max_y) {
            Ok((x, y)) => (x, y),
            Err(_e) => {
                // next position is out of bounds
                return false;
            }
        };

        if matrix[next_y][next_x] == OBSTACLE {
            if visited_obstacles.contains(&(next_x, next_y, cur_dir)) {
                return true;
            }

            visited_obstacles.push((next_x, next_y, cur_dir));

            cur_dir = get_next_direction(&cur_dir);
            continue;
        }

        // move onto the next position
        cur_x = next_x;
        cur_y = next_y;
    }
}

fn find_start(matrix: &Vec<Vec<String>>) -> (usize, usize) {
    for (y, row) in matrix.iter().enumerate() {
        for (x, col_val) in row.iter().enumerate() {
            if col_val != OPEN && col_val != OBSTACLE {
                return (x, y);
            }
        }
    }
    // start at 0, 0 by default
    return (0, 0);
}

fn get_next_pos(
    d: &Direction,
    x: &usize,
    y: &usize,
    max_x: &usize,
    max_y: &usize,
) -> Result<(usize, usize), String> {
    let error_str = String::from("out of bounds");
    match d {
        Direction::Up => {
            if *y == 0 {
                return Err(error_str);
            }
            Ok((*x, y - 1))
        }
        Direction::Right => {
            if x == max_x {
                return Err(error_str);
            }
            Ok((x + 1, *y))
        }
        Direction::Down => {
            if y == max_y {
                return Err(error_str);
            }
            Ok((*x, y + 1))
        }
        Direction::Left => {
            if *x == 0 {
                return Err(error_str);
            }
            Ok((x - 1, *y))
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

// Implement the Display trait for the Direction enum
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Right => write!(f, "Right"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

fn get_next_direction(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => return Direction::Up,
    }
}
