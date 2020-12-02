#![allow(dead_code)]
#![allow(unused_imports)]
mod advent;
use std::time::Instant;
fn main() {
    for (i,e) in advent::challenges().iter().enumerate() {
        
        let start = Instant::now();
        let p1 = (*e).compute_part_one();
        let mid = start.elapsed();
        let p2 = (*e).compute_part_two();
        let end = start.elapsed();

        println!("DAY {}:", i+1);
        println!("\t Part 1: {} {}ms", p1, mid.as_millis());
        println!("\t Part 2: {} {}ms", p2, end.as_millis());
    }
}


