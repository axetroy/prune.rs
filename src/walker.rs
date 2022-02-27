use crate::ruler::Ruler;
use async_recursion::async_recursion;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn get_absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    Ok(absolute_path)
}

#[async_recursion]
pub async fn walk<'a>(dir: PathBuf, ruler: &'a Ruler<'a>) {
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
