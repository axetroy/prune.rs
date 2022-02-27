use crate::ruler::Ruler;
use async_recursion::async_recursion;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn get_absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let r = env::current_dir();
        if r.is_err() {
            Err(r.err().unwrap())
        } else {
            Ok(r.unwrap().join(path))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use crate::walker::get_absolute_path;

    #[test]
    fn test_get_absolute_path() {
        let p1 = PathBuf::new();
        let r1 = get_absolute_path(p1);

        assert!(!r1.is_err());
        assert!(r1.is_ok());

        assert_eq!(
            r1.unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .as_str()
                .trim_end_matches("/"),
            env::current_dir().ok().unwrap().to_str().unwrap()
        );

        let mut p2 = PathBuf::new();
        p2.push("__test__");
        let r2 = get_absolute_path(p2);

        assert!(!r2.is_err());
        assert!(r2.is_ok());

        assert_eq!(
            r2.unwrap().to_str().unwrap(),
            env::current_dir()
                .ok()
                .unwrap()
                .join("__test__")
                .to_str()
                .unwrap()
        );
    }
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
