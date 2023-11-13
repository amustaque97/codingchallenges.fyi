use clap::Parser;
use std::collections::HashSet;
use std::io::Read;

mod radix_sort;
mod merge_sort;

#[derive(Parser, Debug)]
struct Args {
    /// Unique keys.  Suppress all lines that have a key that is equal to an already processed one.  This
    /// option, similarly to -s, implies a stable sort.  If used with -c or -C, sort also checks that
    /// there are no lines with duplicate keys.
    #[arg(short)]
    unique: bool,

    /// File to be sorted
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file_name = args.file.unwrap();
    let unqiue_flag = args.unique;
    let content: String = std::fs::read_to_string(&file_name)
        .unwrap()
        .parse()
        .unwrap_or_else(|_| panic!("Unable to read file {}", &file_name));

    let mut words: Vec<&str> = content.split_whitespace().collect::<Vec<&str>>();
    let set: HashSet<&str> = HashSet::from_iter(words.clone().into_iter());
    if unqiue_flag {
        words = Vec::from_iter(set)
    };
    words.sort();
    for word in words {
        println!("{word}");
    }
}
