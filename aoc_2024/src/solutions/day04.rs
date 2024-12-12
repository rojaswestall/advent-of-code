use crate::utils::file;

const DAY_4_INPUT: &str = "src/input/day04.txt";

pub fn solve() {
    println!("DAY 4 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> i32 {
    const SEARCH_STRING: &str = "XMAS";
    let matrix = file::get_string_matrix_from_text_input(DAY_4_INPUT);
    let mut instances_of_xmas = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, column_val) in row.iter().enumerate() {
            if column_val == &"X" {
                instances_of_xmas +=
                    get_num_instances_of_string_in_all_directions(&SEARCH_STRING, &matrix, j, i)
            }
        }
    }

    return instances_of_xmas;
}

fn part2() -> i32 {
    let matrix = file::get_string_matrix_from_text_input(DAY_4_INPUT);
    let mut instances_of_xmas = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, column_val) in row.iter().enumerate() {
            if column_val == &"A" {
                if is_a_center_of_an_x_mas(&matrix, j, i) {
                    instances_of_xmas += 1
                }
            }
        }
    }

    return instances_of_xmas;
}

enum Direction {
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp,
}

fn get_next_coordinates(
    x: usize,
    y: usize,
    max_vertical: usize,
    max_horizontal: usize,
    d: &Direction,
) -> Result<(usize, usize), String> {
    let error_str = String::from("out of bounds");
    match d {
        Direction::Up => {
            if y == 0 {
                return Err(error_str);
            }
            Ok((x, y - 1))
        }
        Direction::UpRight => {
            if y == 0 || x == max_horizontal {
                return Err(error_str);
            }
            Ok((x + 1, y - 1))
        }
        Direction::Right => {
            if x == max_horizontal {
                return Err(error_str);
            }
            Ok((x + 1, y))
        }
        Direction::RightDown => {
            if x == max_horizontal || y == max_vertical {
                return Err(error_str);
            }
            Ok((x + 1, y + 1))
        }
        Direction::Down => {
            if y == max_vertical {
                return Err(error_str);
            }
            Ok((x, y + 1))
        }
        Direction::DownLeft => {
            if y == max_vertical || x == 0 {
                return Err(error_str);
            }
            Ok((x - 1, y + 1))
        }
        Direction::Left => {
            if x == 0 {
                return Err(error_str);
            }
            Ok((x - 1, y))
        }
        Direction::LeftUp => {
            if x == 0 || y == 0 {
                return Err(error_str);
            }
            Ok((x - 1, y - 1))
        }
    }
}

fn is_s_or_m(s: &String) -> bool {
    return s == "S" || s == "M";
}

fn get_letter_in_direction(
    m: &Vec<Vec<String>>,
    d: &Direction,
    x: usize,
    y: usize,
) -> Result<String, String> {
    let (next_x, next_y) = match get_next_coordinates(x, y, m.len() - 1, m[0].len() - 1, d) {
        Ok((x, y)) => (x, y),
        Err(e) => return Err(e),
    };
    Ok(m[next_y][next_x].clone())
}

fn is_a_center_of_an_x_mas(m: &Vec<Vec<String>>, x: usize, y: usize) -> bool {
    // check top left to bottom right
    let top_left_letter = match get_letter_in_direction(m, &Direction::LeftUp, x, y) {
        Ok(s) => s,
        Err(_e) => return false,
    };
    if !is_s_or_m(&top_left_letter) {
        return false;
    }

    let bottom_right_letter = match get_letter_in_direction(m, &Direction::RightDown, x, y) {
        Ok(s) => s,
        Err(_e) => return false,
    };
    if !is_s_or_m(&bottom_right_letter) || bottom_right_letter == top_left_letter {
        return false;
    }

    // check bottom left to top right
    let bottom_left_letter = match get_letter_in_direction(m, &Direction::DownLeft, x, y) {
        Ok(s) => s,
        Err(_e) => return false,
    };
    if !is_s_or_m(&bottom_left_letter) {
        return false;
    }

    let top_right_letter = match get_letter_in_direction(m, &Direction::UpRight, x, y) {
        Ok(s) => s,
        Err(_e) => return false,
    };
    if !is_s_or_m(&top_right_letter) || top_right_letter == bottom_left_letter {
        return false;
    }

    return true;
}

fn does_matrix_contain_string_from_point(
    s: &str,
    m: &Vec<Vec<String>>,
    d: Direction,
    x: usize,
    y: usize,
) -> bool {
    if s.len() == 0 {
        return true;
    }

    let first_ch = s.chars().next().unwrap().to_string();

    if m[y][x] != first_ch {
        return false;
    }

    if m[y][x] == first_ch && s.len() == 1 {
        return true;
    }

    let (next_x, next_y) = match get_next_coordinates(x, y, m.len() - 1, m[0].len() - 1, &d) {
        Ok((x, y)) => (x, y),
        Err(_e) => return false,
    };

    return does_matrix_contain_string_from_point(&s[1..], m, d, next_x, next_y);
}

fn get_num_instances_of_string_in_all_directions(
    s: &str,
    m: &Vec<Vec<String>>,
    x: usize,
    y: usize,
) -> i32 {
    let mut num_instances = 0;
    if does_matrix_contain_string_from_point(s, m, Direction::Up, x, y) {
        num_instances += 1
    }
    if does_matrix_contain_string_from_point(s, m, Direction::UpRight, x, y) {
        num_instances += 1
    }
    if does_matrix_contain_string_from_point(s, m, Direction::Right, x, y) {
        num_instances += 1
    }
    if does_matrix_contain_string_from_point(s, m, Direction::RightDown, x, y) {
        num_instances += 1
    }
    if does_matrix_contain_string_from_point(s, m, Direction::Down, x, y) {
        num_instances += 1
    }
    if does_matrix_contain_string_from_point(s, m, Direction::DownLeft, x, y) {
        num_instances += 1
    }
    if does_matrix_contain_string_from_point(s, m, Direction::Left, x, y) {
        num_instances += 1
    }
    if does_matrix_contain_string_from_point(s, m, Direction::LeftUp, x, y) {
        num_instances += 1
    }
    return num_instances;
}
