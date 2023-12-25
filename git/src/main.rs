use clap::{Parser, Subcommand, ValueHint};
use std::{env, fs, io::Write, path::PathBuf};

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
            upate_description_repo();
        }
    };
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
