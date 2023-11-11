// use std::io::Write;

use std::fs;
use std::io::{Read, Write};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Use delim as the field delimiter character instead of the tab character.
    #[arg(short)]
    delimeter: Option<String>,

    /// The list specifies fields, separated in the input by the field delimiter character (see the -d option).  Output fields are separated by a single occurrence of the field delimiter character.
    #[arg(short = 'f')]
    field: Option<String>,

    /// File to which cut to be performed
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file: String = args.file.unwrap_or("-".to_string());
    let mut content: String = String::new();
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
    let mut delim: Option<String> = None;
    if args.delimeter.is_some() {
        delim = args.delimeter;
    }

    let field_arg: String = args.field.unwrap(); //.clone();
    let fields = if field_arg.contains(",") {
        field_arg.split(",")
    } else {
        field_arg.split(" ")
    };
    let fields_in_num: Vec<usize> = fields.map(|x| x.parse::<usize>().unwrap()).collect();
    // dbg!(&fields_in_num);

    for line in content.lines() {
        let arr: Vec<&str> = if delim.is_some() {
            line.split(&delim.clone().unwrap()).collect()
        } else {
            delim = Some("\t".to_string());
            line.split_whitespace().collect()
        };
        for f in fields_in_num.clone().into_iter() {
            print!("{}{}", arr[f-1], delim.clone().unwrap());
        }
        println!();
    }
    // std::io::stdout().flush();
}
