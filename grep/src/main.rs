use std::{
    fs::{self, DirEntry},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::exit,
};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    /// Recursively search subdirectories listed.  (i.e., force grep to behave as rgrep).
    recursive: bool,


    #[arg(short='v', long)]
    /// Selected lines are those not matching any of the specified patterns.
    invert_match: bool,

    /// keyword to look for
    pattern: String,

    file: Option<String>,
}

fn find_match(pattern: String, file: PathBuf, recursive: bool) -> bool {
    if file.extension().unwrap_or_default() == "zip" || file.starts_with("./target") {
        return false;
    }
    let content = fs::read_to_string(&file).unwrap();
    let re = Regex::new(format!(".*{}.*", pattern).as_str()).unwrap();
    let matches: Vec<_> = re.find_iter(&content).map(|m| m.as_str()).collect();
    for ele in matches.iter() {
        if recursive {
            println!("{}:{}", file.to_str().unwrap(), ele);
        } else {
            println!("{}", ele);
        }
    }

    if !matches.is_empty() {
        return true;
    }

    return false;
}

// walking a directory only visiting files
fn visit_dirs(dir: &Path, pattern: String, recursive: bool, found: &mut bool) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, pattern.clone(), recursive, found);
            } else {
                // find pattern match
                let result = find_match(pattern.clone(), entry.path(), recursive);
                if result == true {
                    *found = true;
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let pattern: String = args.pattern;
    let file: String = args.file.unwrap_or("-".to_string());
    let mut content = String::new();
    let mut found = false;
    match file.as_str() {
        "-" => {
            let mut stdin = std::io::stdin();
            let _ = stdin.lock();
            let mut buf = Vec::new();
            stdin.read_to_end(&mut buf).unwrap();
            content = String::from_utf8(buf).unwrap();
        }
        "*" => visit_dirs(Path::new("."), pattern.clone(), args.recursive, &mut found),
        _ => content = fs::read_to_string(&file).unwrap().parse().unwrap(),
    }

    if pattern.is_empty() {
        print!("{}", content);
    } else if !args.recursive {
        found = find_match(pattern, file.into(), false);
    }

    if !found {
        exit(1);
    }
}
