use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn get_next_pos(
        &self,
        x: &usize,
        y: &usize,
        max_x: &usize,
        max_y: &usize,
    ) -> Result<(usize, usize), String> {
        let error_str = String::from("out of bounds");
        match self {
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