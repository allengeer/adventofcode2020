use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub struct AdventDay11;
use super::*;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn build_next(seating_plan: &Vec<String>, max_occupy: usize, surfn: &dyn Fn(&Vec<String>, i32, i32) -> String) -> Vec<String> {
    let mut next_state = vec![];

    for i in 0..seating_plan.len(){
        let mut row_text = String::from("");
        for j in 0..seating_plan[i].len() {
            row_text.push(get_next_state(seating_plan, i as i32, j as i32, max_occupy, surfn))
        }
        next_state.push(row_text);
    }
    next_state
}
pub fn get_next_state(seating_plan: &Vec<String>, row: i32, col: i32, max_occupy: usize, surfn: &dyn Fn(&Vec<String>, i32, i32) -> String) -> char {
    let surrounding = surfn(seating_plan, row, col);
     match value_at(seating_plan, row, col) {
        Some('.') => return '.',
        Some('L') => {
            return if surrounding.contains("#") {
                'L'
            } else {
                '#'
            }
        },
        Some('#') => {
            return if surrounding.matches('#').count() >= max_occupy {
                'L'
            } else {
                '#'
            }
        },
        _ => panic!("Shouldnt happen"),
    }
}


pub fn value_at(seating_plan: &Vec<String>, row: i32, col: i32) -> Option<char> {
    // println!("{} {}", row, col);
    if row >= 0 && row < seating_plan.len() as i32  {
        if col >= 0 && col < seating_plan[0].len() as i32 {
            return Some(seating_plan[row as usize].as_bytes()[col as usize]as char);
        }
    }
    None
}

pub fn get_surrounding(seating_plan: &Vec<String>, row: i32, col: i32) -> String {
    let mut ret = String::from("");
    for i in row-1..row+2 {
        for j in col-1..col+2 {
            if i != row || j != col {
                ret.push(value_at(seating_plan, i, j).unwrap_or_default())
            }
        }
    }
    ret
}
pub fn get_surrounding_los(seating_plan: &Vec<String>, row: i32, col: i32) -> String {
    let mut ret = String::from("");
    for i in -1..2 {
        for j in -1..2 {
            if i != 0 || j != 0 {
                let (mut cr,mut cc) = (row+i, col+j);
                while cr >= 0 && cr < seating_plan.len() as i32 && cc >= 0 && cc < seating_plan[0].len()  as i32 {
                    if value_at(seating_plan, cr, cc).unwrap_or_default() != '.' {
                        ret.push(value_at(seating_plan, cr, cc).unwrap_or_default());
                        cr = -1;
                    } else {
                        cr += i;
                        cc += j;
                    }
                }
            }
        }
    }
    ret
}
pub fn count_occupied(seating_map: &Vec<String>) -> usize {
    seating_map.iter().fold(0, |acc, i| i.matches("#").count() + acc)
}

impl super::AdventChallenge for AdventDay11 {
    fn compute_part_one(&self) -> String {
        let mut seating_plan: Vec<String> = lines_from_file("inputs/day11_1.txt");
        let mut next_stage = build_next(&seating_plan, 4, &get_surrounding);
        while calculate_hash(&seating_plan) != calculate_hash(&next_stage) {
            seating_plan = next_stage;
            next_stage = build_next(&seating_plan, 4, &get_surrounding);
        }
        count_occupied(&seating_plan).to_string()
    }
    fn compute_part_two(&self) -> String {
        let mut seating_plan: Vec<String> = lines_from_file("inputs/day11_1.txt");
        let mut next_stage = build_next(&seating_plan, 4, &get_surrounding_los);
        while calculate_hash(&seating_plan) != calculate_hash(&next_stage) {
            seating_plan = next_stage;
            next_stage = build_next(&seating_plan, 5, &get_surrounding_los);
        }
        count_occupied(&seating_plan).to_string()
    }
}
