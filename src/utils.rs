pub mod list_conv {
    pub fn parse_list_of_int(s: &str) -> Vec<i32> {
        let mut o: Vec<i32> = vec![];
        for e in s.split_whitespace() { 
            o.push(e.parse::<i32>().unwrap()); 
        }
        o
    }
    pub fn parse_list_of_str(s: &str) -> Vec<&str> {
        s.split_whitespace().collect()
    }
}
