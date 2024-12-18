use crate::utils::file;

const DAY_9_INPUT: &str = "src/input/day09.txt";

pub fn solve() {
    println!("DAY 9 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

const SPACE: &str = ".";

fn part1() -> i64 {
    let disk_map = get_disk_map_as_vector();
    let expanded_disk_map = expand_disk_map(disk_map);
    let left_compressed_map = move_blocks_left(&expanded_disk_map);

    return compute_checksum(left_compressed_map);
}

fn part2() -> i64 {
    let disk_map = get_disk_map_as_vector();
    let expanded_disk_map = expand_disk_map(disk_map);
    let left_compressed_map = move_file_blocks_left(&expanded_disk_map);

    return compute_checksum(left_compressed_map);
}

fn move_file_blocks_left(expanded_disk_map: &Vec<String>) -> Vec<String> {
    let mut left_compressed_disk_map = expanded_disk_map.clone();
    let mut rev_iter = expanded_disk_map.len() - 1; 
    loop {
        // loop backward
        if rev_iter == 0 {
            break;
        }

        let val  = &expanded_disk_map[rev_iter].to_string();

        if val == SPACE {
            // if we're looking at a space go onto the next value
            rev_iter -= 1;
            continue;
        }

        // compute file size and move rev_iter
        let mut file = vec![val.clone()];
        loop {
            if rev_iter-1 > 0 && &expanded_disk_map[rev_iter-1] == val {
                rev_iter -= 1;
                file.push(expanded_disk_map[rev_iter].to_string());
                continue;
            }
            break;
        }

        let file_size = file.len();
        match get_soonest_open_space(&left_compressed_disk_map, &file_size) {
            Ok(value) =>  {
                if value < rev_iter {
                    left_compressed_disk_map.splice(value..value+file_size, file);
                    // replace the file with spaces
                    left_compressed_disk_map.splice(rev_iter..rev_iter+file_size, vec![SPACE.to_string(); file_size]);
                }
            },
            Err(_err) => {}
        }

        rev_iter -= 1;
    }

    return left_compressed_disk_map;
}

fn get_soonest_open_space(disk_map: &Vec<String>, desired_size: &usize) -> Result<usize, String> {
    let mut space_size = 0;

    // get spaces
    for (i, val) in disk_map.iter().enumerate() {
        if val == SPACE {
            space_size += 1;
        } else {
            space_size = 0;
        }

        if &space_size == desired_size {
            return Ok(i - desired_size + 1);
        }
    }

    return Err("no space fits".to_string());
}

fn compute_checksum(left_compressed_disk_map: Vec<String>) -> i64 {
    let mut checksum = 0;
    for (i, val) in left_compressed_disk_map.iter().enumerate() {
        match val.parse::<i64>() {
            Ok(num) => checksum += num * (i as i64),
            Err(_) => continue,
        }
    }
    return checksum
}

fn move_blocks_left(expanded_disk_map: &Vec<String>) -> Vec<String> {
    let mut left_compressed_disk_map: Vec<String> = Vec::new();
    let mut rev_iter = expanded_disk_map.len() - 1;
    let mut iter = 0;
    
    'outer: for val in expanded_disk_map {
        if iter > rev_iter {
            break;
        }

        if val == SPACE {
            let mut next_val = &expanded_disk_map[rev_iter];
            rev_iter -= 1;

            if next_val == SPACE {
                loop {
                    if iter >= rev_iter {
                        break 'outer;
                    }

                    next_val = &expanded_disk_map[rev_iter];
                    rev_iter -= 1;

                    if next_val != SPACE {
                        break;
                    }
                }
            }

            left_compressed_disk_map.push(next_val.to_string());
        } else {
            left_compressed_disk_map.push(val.to_string());
        }

        iter += 1;
    }

    return left_compressed_disk_map;
}

// expand_disk_map returns the expanded disk map and a list of indexes of spaces
fn expand_disk_map (disk_map: Vec<i32>) -> Vec<String> {
    let mut expanded_disk_map = Vec::new();
    let mut is_space = false;
    let mut cur_index = 0;
    for val in disk_map {

        for _j in 0..val {
            if is_space {
                expanded_disk_map.push(SPACE.to_string());
            } else {
                expanded_disk_map.push(cur_index.to_string());
            }
        }

        if !is_space {
            cur_index += 1;
        }
        
        is_space = !is_space;
    }

    return expanded_disk_map;
}

fn get_disk_map_as_vector() -> Vec<i32> {
    let lines = match file::read_lines_from_text(DAY_9_INPUT) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    if lines.len() > 1 {
        panic!("Detected more than one line in file input");
    }

    let mut disk_map = Vec::new();
    for ch in lines[0].chars() {
        let parsed = ch.to_digit(10).map(|d| d as i32).expect("Error parsing input into i32");
        disk_map.push(parsed);
    }

    return disk_map;
}