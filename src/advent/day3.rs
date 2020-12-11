pub struct AdventDay3 {

}
use super::*;

pub const TREE:char = '#';

pub fn count_trees(slope_x: usize, slope_y: usize, slope: &Vec<String>) -> u64 {
        slope.iter().enumerate().filter(
            |&(i,x)| (x.chars().nth((i/slope_y)*slope_x%x.len()).unwrap() == TREE) && (i%slope_y == 0
            )).count() as u64
}
impl super::AdventChallenge for AdventDay3 {
    fn compute_part_one(&self) -> String {
        let slope = lines_from_file("inputs/day3_1.txt");
        count_trees(3,1,&slope).to_string()
    }
    fn compute_part_two(&self) -> String {
       let slope = lines_from_file("inputs/day3_1.txt");
       (count_trees(1,1,&slope) * 
        count_trees(3,1,&slope) * 
        count_trees(5,1,&slope) *
        count_trees(7,1,&slope) * 
        count_trees(1,2,&slope)).to_string() 
    }
}
