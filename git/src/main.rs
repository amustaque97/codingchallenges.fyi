use clap::{Parser, Subcommand, ValueHint};
use hex;
use sha1::{Digest, Sha1};
use std::{borrow::Cow, env, fs, io::Write, path::PathBuf};

struct GitFile {
    name: &'static str,
    is_dir: bool,
}

impl GitFile {
    pub fn new() -> Vec<GitFile> {
        let files = vec![
            GitFile {
                name: "HEAD",
                is_dir: false,
            },
            GitFile {
                name: "config",
                is_dir: false,
            },
            GitFile {
                name: "description",
                is_dir: false,
            },
            GitFile {
                name: "hooks",
                is_dir: true,
            },
            GitFile {
                name: "info",
                is_dir: true,
            },
            GitFile {
                name: "objects",
                is_dir: true,
            },
            GitFile {
                name: "refs",
                is_dir: true,
            },
        ];

        files
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Initializes an empty git repository.
    Init(InitCtx),

    /// Add files to the staging area
    Add(AddCtx),
}

#[derive(Parser, Debug)]
struct AddCtx {
    /// space separated files to add
    #[clap(default_value = "./", value_hint = ValueHint::FilePath)]
    files: Vec<PathBuf>,
}

#[derive(Parser, Debug)]
struct InitCtx {
    /// Path to the directory to initialize as a repository
    #[clap(default_value = "./", value_hint = ValueHint::DirPath)]
    repository_path: PathBuf,
}

fn main() {
    match Args::parse().command {
        Cmd::Init(ctx) => {
            initialise_empty_repo(ctx.repository_path);
            upate_repo_head();
            upate_description_repo();
        }
        Cmd::Add(ctx) => {
            add_file_cotnent_to_index(ctx.files);
        }
    };
}

fn upate_repo_head() {
    let content = "ref: refs/heads/main";
    let mut file = fs::File::create(".git/HEAD").expect("Unable to open HEAD file!");
    file.write_all(content.as_bytes())
        .expect("Unable to update .git HEAD");
}

fn add_file_cotnent_to_index(files: Vec<PathBuf>) {
    for file in files {
        let hash = calc_hash_object(file);
        println!("{}", hash);
    }
}

fn calc_hash_object(file: PathBuf) -> Cow<'static, str> {
    let content = fs::read_to_string(&file).unwrap();
    let header = format!("blob {}\0", content.as_bytes().len());
    let store = format!("{}{}", header, content);
    calc_content_sha1(store).into()
}

fn calc_content_sha1(store: String) -> Cow<'static, str> {
    let mut hasher = Sha1::new();
    hasher.update(store.as_str());
    let result = hasher.finalize().to_vec();
    hex::encode(result).into()
}

fn upate_description_repo() {
    let mut file = fs::File::create(".git/description").expect("Unable to open description file!");
    file.write_all(
        &"Unnamed repository; edit this file 'description' to name the repository.".as_bytes(),
    )
    .expect("Unable to update repo descriiption");
}

fn initialise_empty_repo(path: PathBuf) {
    if path.to_str().unwrap() != "./" {
        fs::create_dir_all(&path).expect("Unable to initialise empty git repo");
        let _ = env::set_current_dir(&path).is_ok();
    }
    fs::create_dir_all(".git").expect("Unable to initialise empty git repo");
    let files = GitFile::new();
    for f in files {
        let name = format!(".git/{}", f.name);
        if f.is_dir {
            fs::create_dir_all(&name).expect(format!("Unable to create dir: {}", f.name).as_str());
        } else {
            fs::File::create(&name).expect(format!("Unable to create file: {}", f.name).as_str());
        }
    }
}
