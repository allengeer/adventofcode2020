use std::collections::HashSet;
use std::collections::HashMap;

pub struct AdventDay6 {

}
use super::*;


pub fn lines_to_groups(lines: &Vec<String>) -> Vec<Vec<String>> {
	let mut groups: Vec<Vec<String>> = vec![];
	let mut group: Vec<String> = vec![];

	for entry in lines {
		if entry.eq("") {
			groups.push(group);
			group = vec![];
		} else {
			group.push(entry.to_string());
		}
	}
	groups.push(group);
	groups
}

pub fn total_answered( group: &Vec<String> ) -> usize {
	let mut answered = HashSet::new();
	for answer in group {
		for c in answer.chars() {
			answered.insert(c);
		}
	}
	answered.len()
}

pub fn everyone_answered( group: &Vec<String> ) -> usize {
	let mut answered = HashMap::new();
	for answer in group {
		for c in answer.chars() {
			 let counter = answered.entry(c).or_insert(0);
    		*counter += 1;
		}
	}
	answered.iter().filter(|&(_i,e)| *e == group.len() ).count()
}

impl super::AdventChallenge for AdventDay6 {
    fn compute_part_one(&self) -> String {
        let lines = lines_from_file("inputs/day6_1.txt");
        let groups = lines_to_groups(&lines);
        groups.iter().fold(0, |acc, i| 
        	acc + total_answered(i) 
        ).to_string()
    }
    fn compute_part_two(&self) -> String {
    	let lines = lines_from_file("inputs/day6_1.txt");
        let groups = lines_to_groups(&lines);
        groups.iter().fold(0, |acc, i| 
        	acc + everyone_answered(i) 
        ).to_string()
    }
}
