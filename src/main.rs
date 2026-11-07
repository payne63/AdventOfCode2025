use crate::{base::FilesDataEnum, day1::resolve_part1};

mod base;
mod day1;

fn main() {
    match resolve_part1(FilesDataEnum::Day1) {
        Ok(result) => println!("le resultat Day1 part1 est {result}"),
        Err(error) => println!("erreur -> {error}"),
    }
}
