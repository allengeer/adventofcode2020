use std::collections::HashMap;
pub struct AdventDay10;
use super::*;

pub fn n_paths_to_end(rest: &[u64], cache: &mut HashMap<u64,u64>) -> u64 {
    // println!("{:?}", rest);
    if cache.contains_key(rest.first().unwrap() ) {
        // println!("\tPaths Cache Hit {:?} => {:?}", rest, *cache.get(rest.first().unwrap()).unwrap());
        return *cache.get(rest.first().unwrap()).unwrap();
    }
    else {
        if rest.len() == 1 {
            // println!("\tPaths {:?} => 1", rest);
            cache.insert(*rest.first().unwrap(), 1);
            return 1;
        } else {
            let mut count = 0u64;
            let mut niter = rest.iter();
            niter.next();
            for (i, next) in niter.enumerate() {
                if next-rest.first().unwrap() <= 3 {
                    let cn = n_paths_to_end(&rest[(i + 1)..], cache);
                    count += cn;
                } else {
                    break;
                }
            }
            cache.insert(*rest.first().unwrap(), count);
            // println!("\tPaths {:?} => {:?}", rest, count);
            return count;
        }
    }
}

impl super::AdventChallenge for AdventDay10 {
    fn compute_part_one(&self) -> String {
        let mut numbers: Vec<u64> = lines_from_file("inputs/day10_1.txt").iter().map(|x| x.parse::<u64>().unwrap()).collect();
        let mut counts = HashMap::new();
        numbers.push(0u64);
        numbers.push(*numbers.iter().max().unwrap() + 3);

        numbers.sort();
        // println!("{:#?}", numbers);

        let mut niter = numbers.iter();
        let mut source: u64 = *niter.next().unwrap();
        for adapter in niter {
            let step = counts.entry(adapter-source).or_insert(0);
            *step += 1;
            source = *adapter;
            // println!("{:#?}", counts);
        }
        (counts.get(&1).unwrap() * (counts.get(&3).unwrap())).to_string()

    }
    fn compute_part_two(&self) -> String {
        let mut numbers: Vec<u64> = lines_from_file("inputs/day10_1.txt").iter().map(|x| x.parse::<u64>().unwrap()).collect();
        numbers.push(0u64);
        numbers.push(*numbers.iter().max().unwrap() + 3);

        numbers.sort();
        let mut cache = HashMap::new();
        n_paths_to_end(&numbers[..], &mut cache).to_string()
    }
}
