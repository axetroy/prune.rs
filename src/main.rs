use async_recursion::async_recursion;
use clap::{Arg, Command};
use futures::executor;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
mod ruler;
use ruler::{parse_rules, Ruler};

pub fn get_absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
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
                    println!("{}", get_absolute_path(p).unwrap().to_str().unwrap());
                    if !(*ruler.check_only) {
                        fs::remove_dir_all(path.path().as_path()).unwrap()
                    }
                    continue;
                }

                walk(p, ruler).await;
            } else if ruler.file.contains(&name.to_string()) {
                println!("{}", get_absolute_path(p).unwrap().to_str().unwrap());
                if !(*ruler.check_only) {
                    fs::remove_file(path.path().as_path()).unwrap()
                }
                continue;
            }
        }
    }
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

    let mut rulers = parse_rules();

    // You can check the value provided by positional arguments, or option arguments
    if matches.is_present("remove") {
        rulers.check_only = &false;
    } else {
        rulers.check_only = &true;
    }

    let f = walk(root, &rulers);

    executor::block_on(f);
}
