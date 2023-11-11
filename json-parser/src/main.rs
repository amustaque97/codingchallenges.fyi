use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// File to read content from
    file: String,
}

struct Json {
    stack: Vec<String>,
    keywords: Vec<String>,
}

impl Json {
    fn new() -> Self {
        Json {
            stack: Vec::new(),
            keywords: vec!["null".to_string()],
        }
    }

    fn parse(&mut self, input: String) -> bool {
        if input.is_empty() {
            return false;
        }

        let mut ch = input.chars();
        // println!("{:?}", ch);

        while let Some(ele) = ch.next() {
            // println!("{} ", ele);
            match ele {
                '{' | '[' => {
                    self.stack.push(String::from(ele));
                }
                '}' | ']' => {
                    self.stack.pop().unwrap();
                }
                '"' => {
                    let mut ele = ch.next().unwrap();
                    // println!("{}", ele);
                    while ele != '"' {
                        ele = ch.next().unwrap();
                    }
                }
                ',' => {
                    let char = ch.next().unwrap();
                    if char != '\n'
                        && char != '"'
                        && self.stack.get(self.stack.len() - 1).unwrap() != "["
                    {
                        break;
                    }
                    continue;
                }
                't' => {
                    let mut true_keyword = String::from(ele);
                    let mut char = ch.next().unwrap();
                    while char != ',' {
                        true_keyword.push(char);
                        char = ch.next().unwrap();
                    }
                    if true_keyword != "true" {
                        break;
                    }
                }
                'f' => {
                    let mut false_keyword = String::from(ele);
                    let mut char = ch.next().unwrap();
                    while char != ',' {
                        false_keyword.push(char);
                        char = ch.next().unwrap();
                    }
                    if false_keyword != "false" {
                        break;
                    }
                }
                'n' => {
                    let mut null_keyword = String::from(ele);
                    let mut char = ch.next().unwrap();
                    while char != ',' {
                        null_keyword.push(char);
                        char = ch.next().unwrap();
                    }
                    if null_keyword != "null" {
                        break;
                    }
                }
                '0'..='9' => {
                    continue;
                }
                ' ' | ':' | '\n' => {
                    continue;
                }
                _ => {
                    eprintln!("{}", ele);
                    break;
                }
            }
        }

        self.stack.is_empty()
    }
}

fn main() {
    let args = Args::parse();

    let content: String = std::fs::read_to_string(args.file).unwrap().parse().unwrap();

    let mut parser = Json::new();

    if !parser.parse(content) {
        std::process::exit(1);
    }
}
