use std::io::{Error, ErrorKind};
use crate::base::{ ResolveDay, get_lines_from_file};
pub struct Day2 {}


impl ResolveDay for Day2 {
    fn resolve_part1(&self, file_name: &String) -> Result<String, Error> {
        let lines = get_lines_from_file(file_name)?;
        let binding = lines.concat();
        let complet_codes: Vec<&str> = binding.split(",").collect();
        let mut ranges: Vec<Vec<String>> = Vec::new();
        for code in complet_codes {
            let split  = code.split("-").collect::<Vec<&str>>();
            let first = split[0].parse::<u64>().expect("impossible d'extraire la premiere partie");
            let second = split[1].parse::<u64>().expect("impossible d'extraire la seconde partie");
            let range = first..second+1;
            let range : Vec<String> = range.into_iter().map(|x| x.to_string()).collect();
            ranges.push(range);
        }
        let binding = ranges.concat();
        let ranges = binding.iter().filter(|x| is_length_pair(x)).collect::<Vec<&String>>();
        let ranges = ranges.iter().filter(|x| is_twice(x)).collect::<Vec<&&String>>();
        
        let sum :u64= ranges
            .iter()
            .map(|x| x.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
            .iter()
            .sum();
        Ok(sum.to_string())
        
    }

    fn resolve_part2(&self, file_name: &String) -> Result<String, Error> {
        let lines = get_lines_from_file(file_name)?;
        let binding = lines.concat();
        let complet_codes: Vec<&str> = binding.split(",").collect();
        let mut ranges: Vec<Vec<String>> = Vec::new();
        for code in complet_codes {
            let split  = code.split("-").collect::<Vec<&str>>();
            let first = split[0].parse::<u64>().expect("impossible d'extraire la premiere partie");
            let second = split[1].parse::<u64>().expect("impossible d'extraire la seconde partie");
            let range = first..second+1;
            let range : Vec<String> = range.into_iter().map(|x| x.to_string()).collect();
            ranges.push(range);
        }
        let ranges = ranges.concat();
        for range in &ranges{
            print!("range = {:?}",range);
            for length in 1usize..=(range.len()/2){
                println!("length = {}", length);
                if is_multiple(range, length) {
                    
                }
            }
            
        }
        
        Ok("nothing".to_string())
    }
}

fn is_length_pair(sequence: &String) -> bool {
    sequence.len()%2 == 0 
}
fn is_twice(sequence: &String) -> bool {
    let (first, second) = sequence.split_at(sequence.len()/2);
    first == second
}
fn is_multiple(sequence: &String, length: usize) -> bool {
    sequence.len()%length == 0
}