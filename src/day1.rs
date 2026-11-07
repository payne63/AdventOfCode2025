use std::io::{Error, ErrorKind};

use crate::base::{self, FilesDataEnum};

pub fn resolve_part1(file_name: FilesDataEnum) -> Result<u32, Error> {
    let lines = base::get_lines_from_file(&file_name.as_string())?;
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

    // start resolve movement
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
        println!("nouvelle position : {position}");
    }

    Ok(count_zero)
}

struct MoveOrder {
    direction: Direction,
    steps: i32,
}

enum Direction {
    Left,
    Right,
}

