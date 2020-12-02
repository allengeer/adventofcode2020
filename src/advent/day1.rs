pub struct AdventDay1 {
}
use super::*;

fn find_pair_multiply(vec: &[i32], val: i32) -> i32 {
    let mut resp = 0;
    for n1 in vec.iter() {
        let dual = val - n1;
        let index_of_dual = vec.iter().position(|x| *x == dual);
        match index_of_dual {
            Some(_)=> resp = dual*n1,
            None => (),
        }
    }
    resp
}
impl super::AdventChallenge for AdventDay1 {
    fn compute_part_one(&self) -> String {
        let mut numbers:Vec<i32> = lines_from_file("inputs/day1_1.txt")
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        numbers.sort();
        find_pair_multiply(&numbers[..], 2020).to_string()
        
    }
    fn compute_part_two(&self) -> String {
        let mut numbers:Vec<i32> = lines_from_file("inputs/day1_1.txt")
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        numbers.sort();
        let mut val = String::from("");
        for (i,e) in numbers.iter().enumerate() {
            let dual = 2020 - e;
            
            let resp = find_pair_multiply(&numbers[i..],dual);
            if resp > 0 {
                val = (e*resp).to_string();
            }
        }
        val
    } 
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day1_1() {
        let l:Vec<i32> = vec![20, 2000];
        assert_eq!(40000, find_pair_multiply(&l, 2020));
    }
}
