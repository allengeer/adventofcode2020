use std::collections::HashMap;

pub struct AdventDay7 {

}
use super::*;

pub fn contains(contains_bag: &str, rulestring: &str, rulemap: &HashMap<&str,&str>) -> bool {
	// println!("{}", rulestring);
	if rulestring.contains(contains_bag) {
		true
	}
	else if rulestring.contains("no other") {
		false
	}
	else {
		rulestring.split(",").fold(false, |acc,item| {
			let mut i = item.trim().split(" ");
			let _amt = i.next().unwrap();
			let bagcolor = i.collect::<Vec<&str>>().join(" ");
			// println!("\t{} {} {}", amt, bagcolor,rulemap.get(&bagcolor[..]).unwrap());
			acc || contains(contains_bag, rulemap.get(&bagcolor[..]).unwrap(), rulemap)
		})
	}
}

// let mut rulecache: HashMap<&str, u128> = HashMap::new();

pub fn count_bags(rulestring: &str, rulemap: &HashMap<&str,&str>, rulecache: &mut HashMap<&str,u128>) -> u128 {
	if rulestring.contains("no other") {
		1
	}
	else {
		rulestring.split(",").fold(0, |acc,item| {
			let mut i = item.trim().split(" ");
			let amt = i.next().unwrap().parse::<u128>().unwrap();
			let bagcolor = i.collect::<Vec<&str>>().join(" ");
			let cb = count_bags(rulemap.get(&bagcolor[..]).unwrap(), rulemap, rulecache);
			acc + amt * cb
		}) + 1
	}
}

impl super::AdventChallenge for AdventDay7 {
    fn compute_part_one(&self) -> String {
    	let lines = lines_from_file("inputs/day7_1.txt");
    	let mut rulesmap = HashMap::new();

    	lines.iter().for_each(|item|  {
    		let mut ruleiter = item.split(" contain "); 
    		let outer = ruleiter.next().unwrap();
    		let ruletext = ruleiter.next().unwrap();
    		rulesmap.insert(outer, ruletext);
    	});

    	rulesmap.iter().filter(|(_outer,innerrule)| contains("shiny gold", innerrule, &rulesmap)).count().to_string()

    	// println!("{:?}", rulesmap);
    }
    fn compute_part_two(&self) -> String {
    	let lines = lines_from_file("inputs/day7_1.txt");
    	let mut rulesmap = HashMap::new();

    	lines.iter().for_each(|item|  {
    		let mut ruleiter = item.split(" contain "); 
    		let outer = ruleiter.next().unwrap();
    		let ruletext = ruleiter.next().unwrap();
    		rulesmap.insert(outer, ruletext);
    	});

    	let mut rulescache: HashMap<&str, u128> = HashMap::new();
    	(count_bags(rulesmap.get("shiny gold").unwrap(), &rulesmap, &mut rulescache) - 1).to_string()
    }
}
