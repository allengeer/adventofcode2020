mod day1;
mod day2;
/*mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
*/
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub trait AdventChallenge {
    fn compute_part_one(&self) -> String; 
    fn compute_part_two(&self) -> String;
}

#[allow(dead_code)]
pub fn challenges() -> Vec<Box<dyn AdventChallenge>> {
    vec![Box::new(day1::AdventDay1{ }),
         Box::new(day2::AdventDay2{ }),
         //Box::new(day3::AdventDay3{ }),
         //Box::new(day4::AdventDay4{ }),
         //Box::new(day5::AdventDay5{ }), 
         //Box::new(day6::AdventDay6{ }),
         //Box::new(day7::AdventDay7{ }),
         //Box::new(day8::AdventDay8{ }),
         //Box::new(day9::AdventDay9{ }),
         //Box::new(day10::AdventDay10{ }), 
    ]
}

#[allow(dead_code)]
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[allow(dead_code)]
pub fn matrix_from_file(filename: impl AsRef<Path>) -> Vec<Vec<String>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("").split_whitespace().enumerate().map(|p| String::from(p.1)).collect())
        .collect()
}

#[allow(dead_code)]
pub fn list_of_n_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lines_from_file() {
        assert_eq!(vec![String::from("12 12 12 34 55 66"), String::from("32 53 5235 53254"), String::from("242 23423 2342")], lines_from_file("inputs/test_input.txt"));
    }

    #[test]
    fn test_matrix_from_file() {
        assert_eq!(vec![12,13,145], list_of_n_from_file("inputs/test_input_2.txt"));
    }


}
