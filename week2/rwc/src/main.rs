use std::fs::File;
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::{env, io};
use std::{fmt, process};

struct Wc {
    filename: String,
    line: usize,
    word: usize,
    char: usize,
}

impl Wc {
    fn new() -> Wc {
        Wc {
            filename: String::new(),
            line: 0,
            word: 0,
            char: 0,
        }
    }
}

impl fmt::Display for Wc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>8}{:>8}{:>8} {}",
            self.line, self.word, self.char, self.filename
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut wc = Wc::new();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    wc.filename = filename.clone();

    let file = File::open(filename).unwrap();
    wc.char = file.metadata().unwrap().len() as usize;

    let file = io::BufReader::new(file);

    for line in file.lines() {
        let line_str = line.unwrap();
        if line_str.len() > 0 {
            let words: Vec<&str> = line_str.split(' ').collect();
            wc.word += words.len();
        }
        wc.line += 1;
    }

    println!("{}", wc);
}
