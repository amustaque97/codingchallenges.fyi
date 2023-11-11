use assert_fs::prelude::*;
use clap::Parser;
use std::fs;
use std::io;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
struct Args {
    /// The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option.
    #[arg(short = 'c', default_value_t = false)]
    bytes: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short, default_value_t = false)]
    lines: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short, default_value_t = false)]
    words: bool,

    /// The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.
    #[arg(short = 'm', default_value_t = false)]
    characters: bool,

    /// File name or stdin
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let args_passed = args.bytes || args.lines || args.words || args.characters;

    let file: String = args.file.unwrap_or("-".to_string());
    let mut content: String = String::new();
    match file.as_str() {
        "-" => {
            let mut stdin = io::stdin();
            let _ = stdin.lock();
            let mut buf = Vec::new();
            stdin.read_to_end(&mut buf).unwrap();
            content = String::from_utf8(buf).unwrap();
        }
        _ => content = fs::read_to_string(&file).unwrap().parse().unwrap(),
    }
    if args.bytes {
        print!("{} ", content.as_bytes().len());
    }
    if args.lines || !args_passed {
        print!("{} ", content.lines().count());
    }
    if args.words || !args_passed {
        let word_count: usize = content.lines().map(|l| l.split_whitespace().count()).sum();
        print!("{} ", word_count);
    }
    if args.characters || !args_passed {
        print!("{} ", content.chars().count());
    }
    if args_passed && file != "-" {
        println!("{}", file);
    } else {
        println!();
    }
}

#[test]
pub(crate) fn test_word_count() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
    file.write_str("Hello World").unwrap();

    let mut cmd = std::process::Command::new("./target/debug/wc-tool");
    cmd.arg("-w").arg(file.as_os_str());
    let output = cmd.output().unwrap();
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("{} {}\n", 2, file.path().to_str().unwrap())
    );
    Ok(())
}
