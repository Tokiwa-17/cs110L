use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut lines:Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        lines.push(line);
    }
    Ok(lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    let mut words_count = 0;
    let mut lines_count = 0;
    let mut chars_count = 0;

    let lines:Vec<String> = read_lines(filename).unwrap();

    lines_count  = lines.len();

    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        for word in words {
            chars_count += word.chars().count();
            words_count += 1;
        }
    }
    println!("{} {} {} {}", words_count, chars_count, lines_count, filename);
}

