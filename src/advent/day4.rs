pub struct AdventDay4 {

}
use std::collections::HashMap;
use super::*;
use regex::Regex;

pub fn parse_passport( passport_file: &Vec<String>) -> Vec<HashMap<String,String>> {
	let mut passports: Vec<HashMap<String,String>> = vec![];

	let mut passport = HashMap::new();
	for (_i,e) in passport_file.iter().enumerate() {
		
		if e.eq("") {
			// println!("\n\n");
			passports.push(passport);
			passport = HashMap::new();
		}
		else {
			// println!("E: {}", e);
			e.split_whitespace().for_each(|x| {passport.insert(x[0..3].to_string(), x[4..].to_string()); () });
		}
	}
	passports.push(passport);
	passports
}

pub fn validate_passport( passport: &HashMap<String,String> ) -> bool {
	// println!("{:?}", passport);
	passport.contains_key("byr") &&
	passport.contains_key("iyr") &&
	passport.contains_key("eyr") &&
	passport.contains_key("hgt") &&
	passport.contains_key("hcl") &&
	passport.contains_key("ecl") &&
	passport.contains_key("pid")
	// passport.contains_key("cid") &&
}

pub fn number_between( min : i32, max : i32 , val : &String) -> bool {
	match val.parse::<i32>() {
			Ok(result) => result >= min && result <= max,
			Err(_e) => false
		}
}

pub fn validate_field( fieldname: &String, fieldvalue: &String) -> bool {
	// println!("{:?}", passport);
	if fieldname.eq("byr") {
		number_between(1920,2002, fieldvalue)
	} else if fieldname.eq("iyr") {
		number_between(2010,2020, fieldvalue)
	} else if fieldname.eq("eyr") {
		number_between(2020,2030, fieldvalue)
	} else if fieldname.eq("hgt") {
		let re = Regex::new(r"\d+cm").unwrap();
		let rein = Regex::new(r"\d+in").unwrap();
		let mut ret = false;
		if re.is_match(fieldvalue) { ret = number_between(150,193, &fieldvalue[..fieldvalue.len()-2].to_owned()) }
		else if rein.is_match(fieldvalue) { ret = number_between(59,76, &fieldvalue[..fieldvalue.len()-2].to_owned()) }
		ret
	} else if fieldname.eq("hcl") {
		let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
		re.is_match(fieldvalue)
	} else if fieldname.eq("ecl") {
		let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
		re.is_match(fieldvalue)		
	} else if fieldname.eq("pid") {
		let re = Regex::new(r"^\d{9}$").unwrap();
		re.is_match(fieldvalue)
	} else if fieldname.eq("cid") {
		true
	} else {
		false
	}
	// passport.contains_key("byr") &&
	// passport.contains_key("iyr") &&
	// passport.contains_key("eyr") &&
	// passport.contains_key("hgt") &&
	// passport.contains_key("hcl") &&
	// passport.contains_key("ecl") &&
	// passport.contains_key("pid")
	// passport.contains_key("cid") &&
}



impl super::AdventChallenge for AdventDay4 {
    fn compute_part_one(&self) -> String {
        let infile = lines_from_file("inputs/day4_1.txt");
        let passports: Vec<HashMap<String,String>> = parse_passport(&infile);
        passports.iter().filter(|x| validate_passport(&x) ).count().to_string()
     	// String::from("Please Implement")   
    }
    fn compute_part_two(&self) -> String {
    	let infile = lines_from_file("inputs/day4_1.txt");
    	let passports: Vec<HashMap<String,String>> = parse_passport(&infile);
     	let valid_and_complete: Vec<_> = passports.iter().filter(|x| validate_passport(&x)).filter(
     		|x| 
     			x.iter().
     			fold(true, 
     				|acc, item| { 
     					// println!("{} {:?} {}", acc, item, validate_field(item.0, item.1));
 
     					acc && validate_field(item.0, item.1)
     				})
     	).collect();
     	let ret = valid_and_complete.len().to_string();

     	// for passport in valid_and_complete {
     	// 	println!("{:?}\n\n", passport);
     	// }
     	ret
     	
    }
}
