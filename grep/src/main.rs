use std::{
    fs,
    io::{Read, Write}, process::exit,
};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    /// keyword to look for
    pattern: String,

    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let pattern: String = args.pattern;
    let file: String = args.file.unwrap_or("-".to_string());
    let mut content = String::new();
    match file.as_str() {
        "-" => {
            let mut stdin = std::io::stdin();
            let _ = stdin.lock();
            let mut buf = Vec::new();
            stdin.read_to_end(&mut buf).unwrap();
            content = String::from_utf8(buf).unwrap();
        }
        _ => content = fs::read_to_string(&file).unwrap().parse().unwrap(),
    }

    if pattern.is_empty() {
        print!("{}", content);
    } else {
        let re = Regex::new(format!(".*{}.*", pattern).as_str()).unwrap();
        let matches: Vec<_> = re.find_iter(&content).map(|m| m.as_str()).collect();
        for ele in matches.iter() {
            println!("{}", ele);
        }

        if matches.is_empty() {
            exit(255);
        }
    }
}
