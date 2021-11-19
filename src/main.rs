use clap::{App, Arg};
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    Ok(absolute_path)
}

fn walk(dir: PathBuf, ruler: &Ruler) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let p = path.unwrap().path();
        let name = p.file_name().unwrap().to_str().unwrap();

        if ruler.ignore.contains(&name) {
            continue;
        }

        if p.is_dir() {
            if ruler.folder.contains(&name) {
                if *ruler.check_only {
                    println!("{}", absolute_path(p).unwrap().to_str().unwrap())
                } else {
                    fs::remove_dir_all(p.as_path()).unwrap()
                }
                continue;
            }

            walk(p, ruler);
        } else if ruler.file.contains(&name) {
            if *ruler.check_only {
                println!("{}", absolute_path(p).unwrap().to_str().unwrap())
            } else {
                fs::remove_file(p.as_path()).unwrap()
            }
            continue;
        }
    }
}

struct Ruler<'a> {
    ignore: Vec<&'a str>,
    folder: Vec<&'a str>,
    file: Vec<&'a str>,
    check_only: &'a bool,
}

fn main() {
    let matches = App::new("prune")
        .version("v0.1.0")
        .author("Axetroy <axetroy.dev@gmail.com>")
        .about("Prune everything")
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .takes_value(false)
                .about("remove file, defaults check only"),
        )
        .arg(Arg::new("ROOT").about("prune dir").required(true).index(1))
        .get_matches();

    let mut ruler = Ruler {
        ignore: vec![".git"],
        folder: vec!["node_modules", "bowerComponents"],
        file: vec![".DS_Store", ".AppleDouble", ".DS_Store"],
        check_only: &true,
    };

    // You can check the value provided by positional arguments, or option arguments
    let root = if let Some(i) = matches.value_of("ROOT") {
        Path::new(i).to_path_buf()
    } else {
        env::current_dir().unwrap()
    };

    // You can check the value provided by positional arguments, or option arguments
    if matches.is_present("remove") {
        ruler.check_only = &false;
    }

    walk(root, &ruler);
}
