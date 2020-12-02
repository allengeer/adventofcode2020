pub struct AdventDay2 {

}
#[allow(dead_code)]
use super::*;

pub fn password_valid(password_row: &Vec<String>) -> bool {
    let mut rulesiter = password_row[0].split("-");

    let min: i32 = rulesiter.next().unwrap().parse::<i32>().unwrap();
    let max: i32 = rulesiter.next().unwrap().parse::<i32>().unwrap();
    let letter: char = password_row[1].chars().nth(0).unwrap();
    let count:i32 = password_row[2].matches(letter).count() as i32;
    
    //println!("{} {}", min, max);
    count <= max && count >= min
}


pub fn password_valid_two(password_row: &Vec<String>) -> bool {
    let mut rulesiter = password_row[0].split("-");

    let first: usize = rulesiter.next().unwrap().parse::<usize>().unwrap();
    let second: usize = rulesiter.next().unwrap().parse::<usize>().unwrap();
    let letter: char = password_row[1].chars().nth(0).unwrap();
    let first_letter: char = password_row[2].chars().nth(first-1).unwrap();
    let second_letter: char = password_row[2].chars().nth(second-1).unwrap();

    //println!("{} {}", first_letter, second_letter);
    (first_letter != letter && second_letter == letter) ||
    (first_letter == letter && second_letter != letter)
}
impl super::AdventChallenge for AdventDay2 {
    fn compute_part_one(&self) -> String {
        let passwords = matrix_from_file("inputs/day2_1.txt");
        let mut count = 0;
        for password_row in passwords.iter() {
            if password_valid(&password_row){ count = count+1 }
        }
        count.to_string()
    }
    fn compute_part_two(&self) -> String {
        let passwords = matrix_from_file("inputs/day2_1.txt");
        let mut count = 0;
        for password_row in passwords.iter() {
            if password_valid_two(&password_row){ count = count+1 }
        }
        count.to_string()
    }
}
