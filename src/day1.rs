use crate::input::ReadError;
use enumerable::Enumerable;
use seq_macro::seq;
use std::{fs, path::PathBuf};
use thiserror::Error;

// 0 through 99 in order
// if the dial were pointing at 11,
// a rotation of R8 would cause the dial to point at 19.
// After that, a rotation of L19 would cause it to point at 0.

// dial is a circle, turning the dial left from 0 one click makes it point at 99.
// Similarly, turning the dial right from 99 one click makes it point at 0.

// enum Lock99 {
//     _1,
//     _2,
// }

// lock!(0,99);
// lock!(start,end);
macro_rules! lock {
    ( $start:expr,$end:expr ) => {
        // enum Lock {
        // }
        // for i in $start..$end{
        //     println!("{}",i);
        // }
    };
}

seq!(N in 0..=5 {
    #[derive(Debug,Clone,Copy)]
    // Expands to _64, _65, ...
    pub enum Lock {
        #(_~N,)*
    }

    impl Iterator for Lock {
        type Item = Lock;
        fn next(&mut self) -> Option<Self::Item> {
            match self{
                Lock::_0 => Some(Lock::_1),
                Lock::_1 => Some(Lock::_2),
                Lock::_2 => Some(Lock::_3),
                Lock::_3 => Some(Lock::_4),
                Lock::_4 => Some(Lock::_5),
                Lock::_5 => Some(Lock::_0),
            }
        }
    }

    impl DoubleEndedIterator for Lock {
        fn next_back(&mut self) -> Option<Self::Item> {
            match  self{
                Lock::_0 => Some(Lock::_1),
                Lock::_1 => Some(Lock::_2),
                Lock::_2 => Some(Lock::_3),
                Lock::_3 => Some(Lock::_4),
                Lock::_4 => Some(Lock::_5),
                Lock::_5 => Some(Lock::_0),
            }
        }
    }
});

impl Enumerable for Lock {
    // type Enumerator: Iterator<Item = Self>;
    // type Enumerator : Iterator<Item = Lock>
    type Enumerator = Lock; // 
    // type Enumerator = DoubleEndedIterator<Item = Lock>;
    const ENUMERABLE_SIZE_OPTION: Option<usize> = Some(5);

    fn enumerator() -> Self::Enumerator {
        todo!()
    }
}

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
type TurnsOrError = Result<Vec<Turn>, ReadError>;

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
        }) // Iter<Result<Turn, ReadError>>
        .collect::<TurnsOrError>()
}
