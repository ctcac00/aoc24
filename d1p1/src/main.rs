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

fn main() {
    let args: Vec<String> = env::args().collect();
    // check if the user has provided the input file
    if args.len() < 2 {
        eprintln!("Please provide the input file");
        std::process::exit(1);
    }

    let file_path = args[1].clone();
    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
        let result: i32 = 42;
        println!("Result is {:?}", result);
        write_output_file(&result.to_ne_bytes()).unwrap();
    } else {
        eprintln!("Error reading the file {}", file_path);
        std::process::exit(1);
    }
}

