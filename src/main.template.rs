use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = String::from("src/day1-input.txt");
    if let Ok(lines) = read_input(filename) {
        // let result = algo(lines);
        println!("{}", algo(lines));
    }
}

fn algo(input_vect: Vec<String>) -> String {
    let result = String::from("hello");
    
    for line in input_vect {
        println!("{}", line)
    }
    result
}

fn read_input<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut result = vec![];
    for line in lines {
        if let Ok(line_str) = line {
            result.push(line_str);
        }
    }
    Ok(result)
}

