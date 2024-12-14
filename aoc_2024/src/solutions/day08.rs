use std::collections::{HashMap, HashSet};

use crate::utils::file;

const DAY_8_INPUT: &str = "src/input/day08.txt";

pub fn solve() {
    println!("DAY 8 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

const OPEN: &str = ".";

fn part1() -> i32 {
    let matrix = file::get_string_matrix_from_text_input(DAY_8_INPUT);
    let max_x = matrix[0].len();
    let max_y = matrix.len();

    // get nodes
    let nodes = get_nodes(matrix);

    // get antinodes
    let mut anti_nodes = Vec::new();
    for (_key, coords) in &nodes {
        // check each node type at a time
        for (i, coord_1) in coords.iter().enumerate() {
            // compare each node to each other
            for (j, coord_2) in coords.iter().enumerate() {
                if i != j {
                    let a_nodes =
                        get_anti_nodes(coord_1.0, coord_1.1, coord_2.0, coord_2.1, max_x, max_y);
                    // get anti_nodes
                    for n in a_nodes {
                        if !anti_nodes.contains(&(n.0, n.1)) {
                            anti_nodes.push((n.0, n.1));
                        }
                    }
                }
            }
        }
    }

    return anti_nodes.len().try_into().unwrap();
}

// 578 too low
fn part2() -> i32 {
    let matrix = file::get_string_matrix_from_text_input(DAY_8_INPUT);
    let max_x = matrix[0].len();
    let max_y = matrix.len();

    // get nodes
    let nodes = get_nodes(matrix);

    // get antinodes
    let mut anti_nodes = Vec::new();
    for (_key, coords) in &nodes {
        // skip finding anti nodes if there is only one for that key
        if coords.len() == 0 {
            continue;
        }

        // check each node type at a time
        for (i, coord_1) in coords.iter().enumerate() {
            // compare each node to each other
            for (j, coord_2) in coords.iter().enumerate() {
                if i != j {
                    let a_nodes = get_resonant_anti_nodes(
                        coord_1.0, coord_1.1, coord_2.0, coord_2.1, max_x, max_y,
                    );
                    // get anti_nodes
                    for n in a_nodes {
                        if !anti_nodes.contains(&(n.0, n.1)) {
                            anti_nodes.push((n.0, n.1));
                        }
                    }
                }
            }
        }
    }

    return anti_nodes.len().try_into().unwrap();
}

fn get_resonant_anti_nodes(
    x_1: usize,
    y_1: usize,
    x_2: usize,
    y_2: usize,
    x_max: usize,
    y_max: usize,
) -> Vec<(usize, usize)> {
    // add the nodes themselves
    let mut anti_nodes = vec![(x_1, y_1), (x_2, y_2)];

    // recurse to get resonant anti nodes, using anti_nodes as a starting acc
    r_get_res_anti_nodes(&mut anti_nodes, x_1, y_1, x_2, y_2, x_max, y_max);

    return anti_nodes;
}

fn r_get_res_anti_nodes(
    acc: &mut Vec<(usize, usize)>,
    x_1: usize,
    y_1: usize,
    x_2: usize,
    y_2: usize,
    x_max: usize,
    y_max: usize,
) {
    let x_diff = x_1.abs_diff(x_2);
    let y_diff = y_1.abs_diff(y_2);

    let mut node_1_x = 0;
    let mut node_1_y = 0;
    let mut node_1_out_of_bounds = false;
    let mut node_2_x = 0;
    let mut node_2_y = 0;
    let mut node_2_out_of_bounds = false;

    // get nodes' x
    if x_1 < x_2 {
        match x_1.checked_sub(x_diff) {
            Some(res) => node_1_x = res,
            None => node_1_out_of_bounds = true,
        }
        let pos_x = x_2 + x_diff;
        if pos_x >= x_max {
            node_2_out_of_bounds = true;
        } else {
            node_2_x = pos_x;
        }
    } else {
        match x_2.checked_sub(x_diff) {
            Some(res) => node_2_x = res,
            None => node_2_out_of_bounds = true,
        }
        let pos_x = x_1 + x_diff;
        if pos_x >= x_max {
            node_1_out_of_bounds = true;
        } else {
            node_1_x = pos_x;
        }
    }

    // get nodes' y
    if y_1 < y_2 {
        match y_1.checked_sub(y_diff) {
            Some(res) => node_1_y = res,
            None => node_1_out_of_bounds = true,
        }
        let pos_y = y_2 + y_diff;
        if pos_y >= y_max {
            node_2_out_of_bounds = true;
        } else {
            node_2_y = pos_y;
        }
    } else {
        match y_2.checked_sub(y_diff) {
            Some(res) => node_2_y = res,
            None => node_2_out_of_bounds = true,
        }
        let pos_y = y_1 + y_diff;
        if pos_y >= y_max {
            node_1_out_of_bounds = true;
        } else {
            node_1_y = pos_y;
        }
    }

    if !node_1_out_of_bounds && !acc.contains(&(node_1_x, node_1_y)) {
        acc.push((node_1_x, node_1_y));
        r_get_res_anti_nodes(acc, x_1, y_1, node_1_x, node_1_y, x_max, y_max);
    }

    if !node_2_out_of_bounds && !acc.contains(&(node_2_x, node_2_y)) {
        acc.push((node_2_x, node_2_y));
        r_get_res_anti_nodes(acc, x_2, y_2, node_2_x, node_2_y, x_max, y_max);
    }
}

fn get_anti_nodes(
    x_1: usize,
    y_1: usize,
    x_2: usize,
    y_2: usize,
    x_max: usize,
    y_max: usize,
) -> Vec<(usize, usize)> {
    let mut anti_nodes = Vec::new();

    let x_diff = x_1.abs_diff(x_2);
    let y_diff = y_1.abs_diff(y_2);

    let mut node_1_x = 0;
    let mut node_1_y = 0;
    let mut node_1_out_of_bounds = false;
    let mut node_2_x = 0;
    let mut node_2_y = 0;
    let mut node_2_out_of_bounds = false;

    // get nodes' x
    if x_1 < x_2 {
        match x_1.checked_sub(x_diff) {
            Some(res) => node_1_x = res,
            None => node_1_out_of_bounds = true,
        }
        let pos_x = x_2 + x_diff;
        if pos_x >= x_max {
            node_2_out_of_bounds = true;
        } else {
            node_2_x = pos_x;
        }
    } else {
        match x_2.checked_sub(x_diff) {
            Some(res) => node_2_x = res,
            None => node_2_out_of_bounds = true,
        }
        let pos_x = x_1 + x_diff;
        if pos_x >= x_max {
            node_1_out_of_bounds = true;
        } else {
            node_1_x = pos_x;
        }
    }

    // get nodes' y
    if y_1 < y_2 {
        match y_1.checked_sub(y_diff) {
            Some(res) => node_1_y = res,
            None => node_1_out_of_bounds = true,
        }
        let pos_y = y_2 + y_diff;
        if pos_y >= y_max {
            node_2_out_of_bounds = true;
        } else {
            node_2_y = pos_y;
        }
    } else {
        match y_2.checked_sub(y_diff) {
            Some(res) => node_2_y = res,
            None => node_2_out_of_bounds = true,
        }
        let pos_y = y_1 + y_diff;
        if pos_y >= y_max {
            node_1_out_of_bounds = true;
        } else {
            node_1_y = pos_y;
        }
    }

    if !node_1_out_of_bounds {
        anti_nodes.push((node_1_x, node_1_y));
    }

    if !node_2_out_of_bounds {
        anti_nodes.push((node_2_x, node_2_y));
    }

    return anti_nodes;
}

fn get_nodes(matrix: Vec<Vec<String>>) -> HashMap<String, HashSet<(usize, usize)>> {
    let mut nodes: HashMap<String, HashSet<(usize, usize)>> = HashMap::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, col_val) in row.iter().enumerate() {
            if col_val != OPEN {
                nodes
                    .entry(col_val.to_string())
                    .or_insert_with(HashSet::new)
                    .insert((x, y));
            }
        }
    }
    return nodes;
}
