use std::str::FromStr;
use std::collections::HashMap;
use super::*;

pub struct AdventDay8;

#[derive(PartialEq, Debug)]
pub enum ProcessState {
	RUNNING,
	STOPPED,
	PAUSED,
	LOOP
}

#[derive(PartialEq, Debug)]
pub enum Operation {
	Nop(i32),
	Acc(i32),
	Jmp(i32),
}

impl Operation {
	fn from(instruction_text: &str) -> Operation {
		let mut i = instruction_text.split(" ");
		let op = i.next().unwrap();
		match op {
			"nop" => Operation::Nop(i.next().unwrap().trim_start_matches('+').parse::<i32>().unwrap()),
			"acc" => Operation::Acc(i.next().unwrap().trim_start_matches('+').parse::<i32>().unwrap()),
			"jmp" => Operation::Jmp(i.next().unwrap().trim_start_matches('+').parse::<i32>().unwrap()),
			_ => panic!("Unknown Instruction")
		}
	}
}

pub struct Proc {
	ip: usize, 
	acc: i32,
	instructions: Vec<Operation>,
	state: ProcessState,
	exec_count: HashMap<usize, i32>
}

impl Proc {
	pub fn run(&mut self) -> &ProcessState {
		self.state = ProcessState::RUNNING;
		while self.state == ProcessState::RUNNING {
			self.execute();
		}
	    &self.state
	}

	pub fn execute(&mut self) {
		if self.ip < self.instructions.len() {
			let count = self.exec_count.entry(self.ip).or_insert(0);
			*count += 1;
			match count {
				1 => {
					match self.instructions.get(self.ip).unwrap() {
						Operation::Nop(_op) => self.set_ip(self.ip + 1),
						Operation::Acc(acc) => {
							self.set_acc(self.acc + acc);
							self.set_ip(self.ip + 1);
						},
						Operation::Jmp(jmp) => self.set_ip( (self.ip as i32 + jmp) as usize ) ,
					};
				},
				_ => {
					self.state = ProcessState::LOOP;
				}
			}
		} else {
			self.state = ProcessState::STOPPED;
		}
	}

	pub fn set_ip(&mut self, ip: usize) {
		self.ip = ip;
	}

	pub fn set_acc(&mut self, acc: i32) {
		self.acc = acc;
	}

	pub fn reset(&mut self) {
		self.ip = 0;
		self.acc = 0;
		self.exec_count = HashMap::new();
		self.state = ProcessState::STOPPED;
	}

	pub fn flip(&mut self, i: usize) {
		self.instructions.get(i);
		match self.instructions.get(i).unwrap() {
			Operation::Nop(op) => self.instructions[i] = Operation::Jmp(*op),
			Operation::Jmp(jmp) => self.instructions[i] = Operation::Nop(*jmp),
			_ => (),
		};
	}

	pub fn find_flip(&mut self) -> String {
		for i in 0..self.instructions.len() {
			self.reset();
    		self.flip(i);
    		if *self.run() == ProcessState::STOPPED {
    			return self.acc.to_string();
			}
    		self.flip(i);
		}
		return String::from("Not Found");
	}
}

impl super::AdventChallenge for AdventDay8 {
    fn compute_part_one(&self) -> String {
    	let instructions: Vec<Operation> = lines_from_file("inputs/day8_1.txt").iter().map(|x| Operation::from(&x[..])).collect();
    	let mut cpu = Proc {
    		instructions,
    		ip: 0,
    		acc: 0,
    		state: ProcessState::STOPPED,
    		exec_count: HashMap::new()
    	};
    	cpu.run();
    	cpu.acc.to_string()
    }
    fn compute_part_two(&self) -> String {
    	let instructions: Vec<Operation> = lines_from_file("inputs/day8_1.txt").iter().map(|x| Operation::from(&x[..])).collect();
    	let mut cpu = Proc {
    		instructions,
    		ip: 0,
    		acc: 0,
    		state: ProcessState::STOPPED,
    		exec_count: HashMap::new()
    	};
    	cpu.find_flip()
    }
}
