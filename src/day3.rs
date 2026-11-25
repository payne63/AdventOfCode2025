use std::{default, io::{Error, ErrorKind}, path::Display};
use crate::base::{ ResolveDay, get_lines_from_file};
pub struct Day3 {}

#[derive(default::Default)]
struct Best{
    val : usize,
    index : usize
}

impl ResolveDay for Day3 {
    fn resolve_part1(&self, file_name: &String) -> Result<String, Error> {
        let lines = get_lines_from_file(file_name)?;
        let mut best_batterys : Vec<usize> = Vec::new();
        for line in lines{
            let slice: Vec<char> = line.chars().take(line.len()-1).collect();
            let mut i:usize = 0;
            let mut best = Best::default();
            while i < slice.len(){
                let next = slice[i].to_digit(10).expect("conversion impossible") as usize;
                if next > best.val {
                    best.val = next ;
                    best.index = i;
                }
                i+=1;       
            }
            let slice : Vec<char> = line.chars().skip(best.index+1).collect();
            let mut i:usize = best.index;
            let mut best2 = Best::default();
            while i < slice.len(){
                let next = slice[i].to_digit(10).expect("conversion impossible") as usize;
                if next > best2.val {
                    best2.val = next ;
                    best2.index = i;
                }
                i+=1;       
            }
            best_batterys.push(best.val*10 + best2.val);
        }
        let result = best_batterys.iter().fold(0, |x,y| {x+y});
        
        println!("best batterys : {:#?}",best_batterys);
        Ok(result.to_string())
    }

    fn resolve_part2(&self, file_name: &String) -> Result<String, Error> {
        todo!()
    }
}
