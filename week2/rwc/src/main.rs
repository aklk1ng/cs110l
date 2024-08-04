use std::fs::File;
use std::io::BufRead;
use std::process;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let mut line_count: i32 = 0;
    let mut word_count: i32 = 0;
    let mut char_count: i32 = 0;
    for line in io::BufReader::new(file).lines() {
        let line_str = line.unwrap();

        line_count += 1;
        word_count += line_str.split_whitespace().count() as i32;
        char_count += line_str.len() as i32;
    }
    println!(
        "lines: {}, words: {}, characters: {}",
        line_count, word_count, char_count,
    );
}
