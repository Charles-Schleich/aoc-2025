// rust
use std::fs;


// 3rd party

// internal
mod input;


struct MyNum(u8);



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


}
