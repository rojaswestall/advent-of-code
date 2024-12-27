use std::collections::{HashSet,HashMap};
use crate::utils::{direction::Direction, file};

const DAY_12_INPUT: &str = "src/input/day12.txt";

pub fn solve() {
    println!("DAY 12 SOLUTIONS:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

// 1550156
fn part1() -> i64 {
    let matrix = file::get_string_matrix_from_text_input(DAY_12_INPUT);
    let regions = get_regions(matrix);
    let mut total_cost = 0;
    for region in regions {
        total_cost += region.cost();
    }
    return total_cost
}

// 946084
fn part2() -> i64 {
    let matrix = file::get_string_matrix_from_text_input(DAY_12_INPUT);
    let regions = get_regions(matrix);
    let mut total_cost = 0;
    for region in regions {
        total_cost += region.bulk_cost();
    }
    return total_cost
}

struct Region {
    area: i64,
    perimeter: i64,
    boundary_coords: HashSet<(usize, usize, Direction)>
}

impl Region {
    fn cost(&self) -> i64 {
        return self.perimeter * self.area;
    }

    fn bulk_cost(&self) -> i64 {
        return self.get_sides() * self.area;
    }

    fn get_sides(&self) -> i64 {
        let mut directional_coords: HashMap<Direction,Vec<(usize, usize)>> = HashMap::new();

        // Group coordinates by direction
        for &(x, y, d) in &self.boundary_coords {
            directional_coords.entry(d).or_default().push((x, y));
        }

        let mut total_sides = 0;

        for d in directional_coords.keys() {

            let mut directional_grouping: HashMap<usize, Vec<usize>> = HashMap::new();

            // Group coordinates by x (vertical alignment) and y (horizontal alignment)
            for (x, y) in directional_coords[d].clone() {
                if d == &Direction::Up || d == &Direction::Down {
                    // group horizontally
                    directional_grouping.entry(y).or_default().push(x);
                } else {
                    // group vertically
                    directional_grouping.entry(x).or_default().push(y);
                }
            }

            // count sides
            let sides = directional_grouping.values().map(|s| count_segments(s)).sum::<usize>();
    
            total_sides += sides;
        }
    
        return total_sides as i64;
    }
}

fn count_segments(points: &Vec<usize>) -> usize {
    // Sort points and count contiguous segments
    let mut sorted_points = points.clone();
    sorted_points.sort_unstable();

    let mut segments = 0;
    let mut previous = None;

    for &point in &sorted_points {
        if let Some(prev) = previous {
            if point != prev + 1 {
                segments += 1; // New segment starts
            }
        } else {
            segments += 1; // First segment
        }
        previous = Some(point);
    }

    return segments;
}

fn get_regions(matrix: Vec<Vec<String>>) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;

    for (y, row) in matrix.iter().enumerate() {
        for (x, plot_name) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let region_info = get_info_for_region(
                plot_name,
                (x, y),
                &mut visited,
                &matrix,
                (max_x, max_y),
                (0, 0, HashSet::new())
            );

            regions.push(Region { 
                area: region_info.0,
                perimeter: region_info.1,
                boundary_coords: region_info.2
            });
        }
    }

    return regions;
}

fn get_info_for_region(
    region_name: &String,
    pos: (usize, usize),
    visited: &mut HashSet<(usize,usize)>,
    matrix: &Vec<Vec<String>>,
    max: (usize, usize),
    region_info: (i64, i64, HashSet<(usize, usize, Direction)>)
) -> (i64, i64, HashSet<(usize, usize, Direction)>) {
    if visited.contains(&pos) || &matrix[pos.1][pos.0] != region_name {
        return region_info;
    }

    visited.insert(pos);

    // calculate area and perimeter of current plot
    let mut area = region_info.0;
    let mut perimeter: i64 = 0;
    let mut boundaries: HashSet<(usize, usize, Direction)> = HashSet::new();

    let directions  = vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left];
    for d in directions {
        let mut result = region_info.clone();
        match d.get_next_pos(&pos.0, &pos.1, &max.0, &max.1) {
            Ok((x, y)) => {
                // calculate perimeter
                let next = &matrix[y][x];
                if next != region_name {
                    perimeter += 1;
                    boundaries.insert((pos.0, pos.1, d));                    
                }
                // get info for rest of region in this direction
                result = get_info_for_region(region_name, (x, y), visited, matrix, max, region_info.clone());
            },
            Err(_e) => {
                // out of bounds
                perimeter += 1;
                boundaries.insert((pos.0, pos.1, d));
            }
        };
        area += result.0;
        perimeter += result.1;
        boundaries.extend(result.2);
    }

    // account for the current position
    area += region_info.0 + 1;

    return (area, perimeter, boundaries);
}