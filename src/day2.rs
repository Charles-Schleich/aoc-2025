use crate::input::ReadError;
use std::{fs, path::PathBuf};

struct IDRange(u64,u64);

type IDRangeOrError = Result<Vec<IDRange>, ReadError>;

fn read_lines_parse_to_ids(file_path: PathBuf) -> IDRangeOrError {
    fs::read_to_string(file_path)?
        .into_iter() //
        .map(|x| {

        })
        .collect::<IDRangeOrError>()
}

// 
fn day2(){

}
