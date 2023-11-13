use clap::Parser;
use std::collections::HashSet;
use std::io::Read;

mod merge_sort;
mod radix_sort;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    /// Use mergesort.  This is a universal algorithm that can always be used, but it is not always the fastest.
    mergesort: bool,

    /// Try to use radix sort, if the sort specifications allow.  The radix sort can only be used for trivial locales (C and POSIX), and it cannot be used for numeric or month sort.  Radix sort is very fast and stable.
    #[arg(long)]
    radixsort: bool,

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

    let mut words: Vec<String> = content
        .split_whitespace()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    let set: HashSet<String> = HashSet::from_iter(words.clone().into_iter());
    if unqiue_flag {
        words = Vec::from_iter(set)
    };
    if args.mergesort {
        words = merge_sort::sort(words);
    } else if args.radixsort {
        words = radix_sort::sort(words);
    } else {
        words.sort();
    }
    for word in words {
        println!("{word}");
    }
}
