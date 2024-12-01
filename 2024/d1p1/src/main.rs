use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_output_file(result: &[u8]) -> std::io::Result<()> {
    let mut f = File::create("output.txt")?;
    f.write_all(result)?;
    Ok(())
}

fn solve(group_1: Vec<i32>, group_2: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..group_1.len() {
        result += group_1[i].abs_diff(group_2[i]);
    }
    result as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // check if the user has provided the input file
    if args.len() < 2 {
        eprintln!("Please provide the input file");
        std::process::exit(1);
    }

    let mut group_1: Vec<i32> = Vec::new();
    let mut group_2: Vec<i32> = Vec::new();

    let file_path = args[1].clone();
    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            let numbers: Vec<&str> = line.split_whitespace().collect();
            println!("{:?}", numbers);

            group_1.push(numbers[0].parse::<i32>().unwrap());
            group_2.push(numbers[1].parse::<i32>().unwrap());
        }
        group_1.sort();
        group_2.sort();

        let result = solve(group_1, group_2);
        println!("Result is {:?}", result);
        if let Err(e) = write_output_file(result.to_string().as_bytes()) {
            eprintln!("Error writing the output file: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("Error reading the file {}", file_path);
        std::process::exit(1);
    }
}