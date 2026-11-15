use crate::base::{ResolveDay, get_file_path};
use crate::day1::Day1;
use crate::day2::Day2;
use std::io::Error;

mod base;
mod day1;
mod day2;

fn main() {
    match resolve_day(){
        Ok(_) => println!("success"),
        Err(error) => println!("erreur -> {}",error),
    }
}

fn resolve_day() -> Result<(), Error> {
    let day = Day1{};
    match day.resolve_part1(&get_file_path("Day1", None)?) {
        Ok(result) => println!("le resultat Day1 part1 est {result}"),
        Err(error) => println!("erreur -> {}",error),
    }
    match day.resolve_part2(&get_file_path("Day1",None)?) {
        Ok(result) => println!("le resultat Day1 part2 est {result}"),
        Err(error) => println!("erreur -> {error}"),
    }
    let day2 = Day2{};
    match day2.resolve_part1(&get_file_path("Day2", None)?) {
        Ok(result) => println!("le resultat Day2 part1 est {result}"),
        Err(error) => println!("erreur -> {error}"),
    }
    match day2.resolve_part2(&get_file_path("Day2", Some(true))?) {
        Ok(result) => println!("le resultat Day2 part2 est {result}"),
        Err(error) => println!("erreur -> {error}"),
    }
    Ok(())
}
