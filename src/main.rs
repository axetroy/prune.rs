use async_recursion::async_recursion;
use clap::{Arg, Command};
use futures::executor;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    Ok(absolute_path)
}

#[async_recursion]
async fn walk<'a>(dir: PathBuf, ruler: &'a Ruler<'a>) {
    if let Ok(paths) = fs::read_dir(dir) {
        for path in paths.flatten() {
            let p = path.path();
            let name = p.file_name().unwrap().to_str().unwrap();

            if ruler.ignore.contains(&name.to_string()) {
                continue;
            }

            if p.is_dir() {
                if ruler.folder.contains(&name.to_string()) {
                    println!("{}", absolute_path(p).unwrap().to_str().unwrap());
                    if !(*ruler.check_only) {
                        fs::remove_dir_all(path.path().as_path()).unwrap()
                    }
                    continue;
                }

                walk(p, ruler).await;
            } else if ruler.file.contains(&name.to_string()) {
                println!("{}", absolute_path(p).unwrap().to_str().unwrap());
                if !(*ruler.check_only) {
                    fs::remove_file(path.path().as_path()).unwrap()
                }
                continue;
            }
        }
    }
}

struct Ruler<'a> {
    ignore: Vec<String>,
    folder: Vec<String>,
    file: Vec<String>,
    check_only: &'a bool,
}

fn parse_rules() -> Ruler<'static> {
    let mut ruler = Ruler {
        ignore: vec![],
        folder: vec![],
        file: vec![],
        check_only: &true,
    };

    let bytes = include_bytes!("../rulers.txt").to_vec();

    let txt = match String::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let lines = txt.lines();

    for mut line in lines {
        line = line.trim();

        if line.is_empty() {
            continue;
        }

        let first = &line.chars().next().unwrap().to_string();

        if first == "#" {
            continue;
        }

        let s = line.find(' ').unwrap();

        if s > 1 {
            panic!("Invalid ruler: {}", line);
        }

        let pattern = &line.trim_start_matches(first).trim().to_string();

        if first == "I" {
            ruler.ignore.push(pattern.to_string());
            continue;
        }

        if first == "D" {
            ruler.folder.push(pattern.to_string());
            continue;
        }

        if first == "F" {
            ruler.file.push(pattern.to_string());
            continue;
        }

        panic!("Invalid ruler: {}", line);
    }

    ruler
}

fn main() {
    let matches = Command::new("prune")
        .bin_name("prune")
        .version("v0.1.0")
        .author("Axetroy <axetroy.dev@gmail.com>")
        .about("Streamline your disk space and delete some unnecessary files")
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .takes_value(false)
                .help("remove file, defaults check only"),
        )
        .arg(
            Arg::new("ROOT")
                .help("The target directory you want to prune")
                .required(true)
                .index(1),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    let root = if let Some(i) = matches.value_of("ROOT") {
        Path::new(i).to_path_buf()
    } else {
        env::current_dir().unwrap()
    };

    let mut ruler = parse_rules();

    // You can check the value provided by positional arguments, or option arguments
    if matches.is_present("remove") {
        ruler.check_only = &false;
    } else {
        ruler.check_only = &true;
    }

    let f = walk(root, &ruler);

    executor::block_on(f);
}
