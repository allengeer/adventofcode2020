//mod utils;
mod advent;

//use utils::list_conv;
//use text_io::read;

fn main() {
    for item in advent::challenges() {
        println!("{:?}", (*item).compute_part_one());
        println!("{:?}", (*item).compute_part_two());
    }
}


