use std::fmt;
pub struct AdventDay5 {

}
use super::*;


#[derive(Debug)]
pub struct Seat {
	row : u64,
	column: u64
}

impl Seat {
	fn seat_id(&self) -> u64 {
		self.row * 8 as u64 + self.column
	}

	pub fn from_boarding_pass(boarding_pass: &String ) -> Seat {
		let mut col_range = (0, 7);
		let mut row_range = (0,127);

		for c in boarding_pass.chars() {
			match c {
				'R' => col_range = (col_range.0 + (col_range.1 - col_range.0)/2 + 1, col_range.1), 
				'L' => col_range = (col_range.0, col_range.0 + (col_range.1 - col_range.0)/2 ), 
				'F' => row_range = (row_range.0, row_range.0 + (row_range.1 - row_range.0)/2 ),
				'B' => row_range = (row_range.0 + (row_range.1 - row_range.0)/2 + 1, row_range.1), 
				_ => println!("Unknown Character")
			};
			// println!("{} => {:?} {:?}", c, col_range, row_range);
		}

		Seat {
			row: row_range.0,
			column: col_range.0
		}
	}
}

impl super::AdventChallenge for AdventDay5 {
    fn compute_part_one(&self) -> String {
        let infile = lines_from_file("inputs/day5_1.txt");

        infile.iter().map(|x| Seat::from_boarding_pass(x).seat_id()).max().unwrap().to_string()
    }
    fn compute_part_two(&self) -> String {
    	let infile = lines_from_file("inputs/day5_1.txt");
    	
    	let mut seat_idsa: Vec<u64> = infile.iter().map(|x| Seat::from_boarding_pass(x).seat_id()).collect();
    	seat_idsa.sort();
    	let mut last = 0;
    	let mut ret = 0;
    	for seat_id in seat_idsa {
    		if last == 0 {
    			last = seat_id;
    		}
    		else if seat_id > last+1 {
    			ret = last+1; 
    		}
    		else {
    			last = seat_id; 
    		}
    	}
    	ret.to_string()
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_seat() {
        let seat = Seat {
        	row: 44,
        	column: 5
        };
        assert_eq!(seat.seat_id(),  357);
    }

    #[test]
    fn test_boardingpass() {
        let seat = Seat::from_boarding_pass(&String::from("FBFBBFFRLR"));
        println!("{:?}", seat);
        assert_eq!(seat.seat_id(),  357);
    }
}