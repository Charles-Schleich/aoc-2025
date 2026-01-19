use std::{fs, num::ParseIntError, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day1Error {
    #[error("Not Left or Right")]
    NotLeftOrRight,
    #[error("Empty Line")]
    EmptyLine,
}


type TurnMax = u16;
#[derive(Debug)]
pub struct Turn {
    pub direction: Direction,
    pub amount: TurnMax,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}
// 
type TurnsOrError = Result<Vec<Turn>,ReadError>;

pub fn read_lines_parse_to_turn(file_path: PathBuf) -> TurnsOrError {
    fs::read_to_string(file_path)?
        .lines() //
        .into_iter() //
        .map(|x| {
            let (head_str, tail) = x.split_at(1);
            let head = head_str.chars().next();

            match (tail.parse::<TurnMax>(), head) {
                (_, None) => Err(ReadError::from(Day1Error::EmptyLine)),
                (Err(e), _) => Err(ReadError::from(e)),
                (Ok(amount), Some(head)) => match head {
                    'L' => Ok(Direction::Left),
                    'R' => Ok(Direction::Right),
                    _ => Err(ReadError::from(Day1Error::NotLeftOrRight)),
                }
                .map(|direction| Turn { amount, direction }),
            }
        })// Iter<Result<Turn, ReadError>>
        .collect::<TurnsOrError>()

}
