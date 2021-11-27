use async_recursion::async_recursion;
use clap::{App, Arg};
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

            if ruler.ignore.contains(&name) {
                continue;
            }

            if p.is_dir() {
                if ruler.folder.contains(&name) {
                    println!("{}", absolute_path(p).unwrap().to_str().unwrap());
                    if !(*ruler.check_only) {
                        fs::remove_dir_all(path.path().as_path()).unwrap()
                    }
                    continue;
                }

                walk(p, ruler).await;
            } else if ruler.file.contains(&name) {
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
    ignore: Vec<&'a str>,
    folder: Vec<&'a str>,
    file: Vec<&'a str>,
    check_only: &'a bool,
}

fn main() {
    let matches = App::new("prune")
        .bin_name("prune")
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
        folder: vec!["node_modules", "bowerComponents", ".cache"],
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

    let f = walk(root, &ruler);

    executor::block_on(f);
}
