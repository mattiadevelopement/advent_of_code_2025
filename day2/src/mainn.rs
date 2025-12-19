use std::fs;




fn main() {
    let contents =  fs::read_to_string("src\\ids").expect("stupid error");
}






let ranges: Vec<&str> = contents.split(",").collect();