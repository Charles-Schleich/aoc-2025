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
// macro_rules! lock {
//     ( $start:expr,$end:expr ) => {
// enum Lock {
// }
// for i in $start..$end{
//     println!("{}",i);
// }
//     };
// }

seq!(N in 0..=99 {
    #[derive(Debug,Clone,Copy)]
    // Expands to _64, _65, ...
    pub enum Lock {
        #(Var~N,)*
    }

    // ITERATOR DOUBLE ENDED APPROACH
    // impl Iterator for Lock {
    //     type Item = Lock;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         match self{
    //             #(_~N,)*
    //             Lock::_~N =>Lock::_~N+1,
    //             //   #()* =>#(Lock::_~N+1)*
    //             // Lock::_0 => Some(Lock::_1),
    //             // Lock::_0 => Some(Lock::_1),
    //             // Lock::_1 => Some(Lock::_2),
    //             // Lock::_2 => Some(Lock::_3),
    //             // Lock::_3 => Some(Lock::_4),
    //             // Lock::_4 => Some(Lock::_5),
    //             // Lock::_5 => Some(Lock::_0),
    //         }
    //     }
    // }

    // impl DoubleEndedIterator for Lock {
    //     fn next_back(&mut self) -> Option<Self::Item> {
    //         match  self{
    //             Lock::_0 => Some(Lock::_1),
    //             Lock::_1 => Some(Lock::_2),
    //             Lock::_2 => Some(Lock::_3),
    //             Lock::_3 => Some(Lock::_4),
    //             Lock::_4 => Some(Lock::_5),
    //             Lock::_5 => Some(Lock::_0),
    //         }
    //     }
    // }

    // MACRO RULES APPROACH
    // macro_rules! next_enum {
    //     ($($current:ident => $next:ident),* $(,)?) => {
    //         impl Lock {
    //             pub fn next(&self) -> Self {
    //                 match self {
    //                     $(Lock::$current => Lock::$next),*,
    //                 }
    //             }
    //         }
    //     };
    // }

    // next_enum! {
    //     Var0 => Var1,
    //     Var1 => Var2,
    //     Var2 => Var3,
    //     Var3 => Var4,
    //     Var98 => Var99,
    //     Var99 => Var0, // Wrap around
    // }
});

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

impl Turn {
    fn new(d: Direction, tm: TurnMax) -> Self {
        Self {
            direction: d,
            amount: tm,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

type TurnsOrError = Result<Vec<Turn>, ReadError>;

fn read_lines_parse_to_turn(file_path: PathBuf) -> TurnsOrError {
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

struct Dial {
    current: u8,
    clicks_at_zero: u16,
    // starting_on_zero: u16,
}

// The dial starts by pointing at 50.
impl Dial {
    fn new() -> Self {
        Self {
            current: 50,
            clicks_at_zero: 0,
        }
    }
    fn val(&self) -> u8 {
        self.current
    }

    pub fn turn(&mut self, turn: Turn) {
        match turn.direction {
            Direction::Left => self.left(turn.amount),
            Direction::Right => self.right(turn.amount),
        }
    }

    // decrease : 11 + L8 = 3
    fn left(&mut self, mut val: u16) {
        // continue subtracting 99 until val is less than 99
        while val > 0 {
            val = val - 1;

            // wrap
            if self.current == 0 {
                self.current = 99;
            } else {
                self.current = self.current - 1;
            }

            if self.current == 0 {
                self.clicks_at_zero += 1;
            }
        }
    }

    // increase : 11 + R8 = 19
    fn right(&mut self, mut val: u16) {
        // continue subtracting 99 until val is less than 99
        while val > 0 {
            val = val - 1;
            // wrap
            if self.current == 99 {
                self.current = 0;
            } else {
                self.current = self.current + 1;
            }

            if self.current == 0 {
                self.clicks_at_zero += 1;
            }
        }
    }
}

pub fn day1() {
    //
    let turns_or_err = read_lines_parse_to_turn(PathBuf::from("./input.txt")).unwrap();
    println!("turns or Err {:?}", turns_or_err.len());

    let mut dial = Dial::new();
    let mut count = 0;
    for turn in turns_or_err {
        dial.turn(turn);
        if dial.val() == 0 {
            count = count + 1;
        };
    }

    println!("Part 1: Ended on zero    {}", count);
    println!("Part 2: Clicks past zero {}", dial.clicks_at_zero);
}

////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_count_start_50_l1000() {
        let turns = vec![
            Turn::new(Direction::Left, 1000), //40
        ];

        let mut dial = Dial::new();
        for turn in turns {
            dial.turn(turn);
        }

        assert_eq!(dial.clicks_at_zero, 10)
    }

    #[test]
    fn test_add() {
        let turns = vec![
            Turn::new(Direction::Left, 10),  //40
            Turn::new(Direction::Left, 10),  //40
            Turn::new(Direction::Right, 10), //40
        ];

        let mut dial = Dial::new();
        let expected_dial_values = vec![40, 30, 40];
        let mut actual_dial_values = Vec::new();
        for turn in turns {
            dial.turn(turn);
            actual_dial_values.push(dial.val());
        }
        assert_eq!(expected_dial_values, actual_dial_values)
    }
}
