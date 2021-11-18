use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn walk(dir: PathBuf, rules: &Vec<&str>) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let p = path.unwrap().path();
        let name = p.file_name().unwrap().to_str().unwrap();

        if p.is_dir() {
            if rules.contains(&name) {
                println!("Found: {}", p.display());
                continue;
            }
            walk(p, &rules);
        } else {
            if rules.contains(&name) {
                println!("Found: {}", p.display());
                continue;
            }
        }
    }
}

fn main() {
    // 规则
    let rules = vec![".git", "node_modules", "bowerComponents", ".gitignore"];

    walk(Path::new("./aaa").to_path_buf(), &rules);
}
