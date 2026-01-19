// rust
use std::path::PathBuf;
// 3rd party

// internal
mod input;
mod day1;
struct MyNum(u8);
// 
impl MyNum {
    fn new(val:u8) -> Self {
        Self(val)
    }

    fn l(mut self, val:u8) {
        // continue subtracting 99 until val is less than 99
        while self.0 > 99 {
            self.0 = self.0 - 99;
        }
    }

    fn r(mut self, val:u8) {
        // continue subtracting 99 until val is less than 99
        while self.0 > 99 {
            self.0 = self.0 - 99;
        }
    }
}

fn main() {
    let turns_or_err = day1::read_lines_parse_to_turn(PathBuf::from("./input.txt"));
    println!("turns or Err {:?}",turns_or_err);
    // 
    for turn in turns_or_err.unwrap() {
        println!("{:?},{:?}",turn.amount,turn.direction);
    }
    

}
