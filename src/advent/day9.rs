pub struct AdventDay9 {

}
use super::*;

pub fn is_valid(numbers: &[u64], val: u64) -> bool {
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            if numbers[i] + numbers[j] == val {
                return true;
            }
        }
    }
    return false;
}

pub fn find_range(val: u64, numbers: &[u64]) -> u64 {
    let mut sum;
    for i in 0..(numbers.len()-1) {
        sum = numbers[i];
        for j in (i+1)..numbers.len() {
            sum += numbers[j];
            if sum == val {
                return *numbers[i..j].iter().max().unwrap() + *numbers[i..j].iter().min().unwrap()
            }
        }
    }
    return 0;
}
impl super::AdventChallenge for AdventDay9 {
    fn compute_part_one(&self) -> String {
        let numbers: Vec<u64> = lines_from_file("inputs/day9_1.txt").iter().map(|x| x.parse::<u64>().unwrap()).collect();
        for i in 25..numbers.len() {
            if !is_valid(&numbers[(i-25)..i], numbers[i]) {
                return numbers[i].to_string()
            }
        }
        String::from("")
    }
    fn compute_part_two(&self) -> String {
        let numbers: Vec<u64> = lines_from_file("inputs/day9_1.txt").iter().map(|x| x.parse::<u64>().unwrap()).collect();
        let mut enc_total: u64 = 0;
        for i in 25..numbers.len() {
            if !is_valid(&numbers[(i-25)..i], numbers[i]) {
                enc_total = numbers[i]
            }
        }
        find_range(enc_total, &numbers[..]).to_string()
    }
}
