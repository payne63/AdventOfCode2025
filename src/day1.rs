use std::io::{Error, ErrorKind};
use crate::base::{self, ResolveDay, get_lines_from_file};

pub struct Day1 {}

impl ResolveDay for Day1 {
    fn resolve_part1(&self, file_name: &String) -> Result<String, Error> {
        let lines = get_lines_from_file(file_name)?;
    // start resolve movement

    let moves = get_move_from_lines(lines)?;

    let mut count_zero = 0u32;
    let mut position = 50i32;
    for movement in moves.into_iter() {
        match movement.direction {
            Direction::Left => position -= movement.steps % 100,
            Direction::Right => position += movement.steps % 100,
        }
        if position < 0 {
            position = 100 + position;
        } else if position > 99 {
            position = position % 100;
        }
        if position == 0 {
            count_zero += 1;
        }
    }

    Ok(count_zero.to_string())

}

    fn resolve_part2(&self, file_name: &String) -> Result<String, Error> {
        let lines = get_lines_from_file(file_name)?;
        let moves = get_move_from_lines(lines)?;
    
        let mut count_zero = 0i32;
        let mut position = 50i32;
        for movement in moves {
            match (movement.direction, movement.steps) {
                (Direction::Left, steps) => {
                    for _ in 0..steps {
                        position -= 1;
                        if position < 0 {
                            position = 100 + position;
                        }
                        if position == 0 {
                            count_zero += 1;
                        }
                    }
                }
                (Direction::Right, steps) => {
                    for _ in 0..steps {
                        position += 1;
                        if position > 99 {
                            position = position % 100;
                        }
                        if position == 0 {
                            count_zero += 1;
                        }
                    }
                }
            }
        }
    
        Ok(count_zero.to_string())
    }
}

fn get_move_from_lines(lines: Vec<String>) -> Result<Vec<MoveOrder>, Error> {
    let mut moves: Vec<MoveOrder> = Vec::new();
    for line in lines {
        let first_char = match line.get(0..1) {
            Some(val) => val,
            None => return Err(Error::new(ErrorKind::InvalidData, "Empty line")),
        };
        let second_part = match line.get(1..) {
            Some(val) => val,
            None => return Err(Error::new(ErrorKind::InvalidData, "Empty line")),
        };
        moves.push(MoveOrder {
            direction: match first_char {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "impossible de cast la direction",
                    ));
                }
            },
            steps: match second_part.parse() {
                Ok(v) => v,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "impossible de cast le nombre de pas",
                    ));
                }
            },
        });
    }
    Ok(moves)
}

struct MoveOrder {
    direction: Direction,
    steps: i32,
}

enum Direction {
    Left,
    Right,
}
