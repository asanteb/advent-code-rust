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
    let mut result = String::from("");
    for (i, line1) in input_vect.iter().enumerate() {
        for (j, line2) in input_vect.iter().enumerate() {
            if i == j {
                continue
            }
            let num1: i32 = line1.parse().unwrap();
            let num2: i32 = line2.parse().unwrap();
            let total = num1 + num2;
            if total == 2020 {
                result = (num1 * num2).to_string();
            }
        }
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

