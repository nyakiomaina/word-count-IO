use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprint!("Usage: wordcount <filename>");
        process::exit(1);
    }

    let filename = &args[1];

    let contents = match read_file_contents(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1)
        }
    };

    let (lines, words, characters) = count_contents(&contents);

    println!("Lines: {}", lines);
    println!("Words: {}", words);
    println!("Characters: {}", characters);

    }

fn count_contents(contents: &str) -> (usize, usize, usize){
    let lines = contents.lines().count();
    let words = contents.split_whitespace().count();
    let characters = contents.chars().count();
    (lines, words, characters)
}

fn read_file_contents(filename: &str)  -> io::Result<String>{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}