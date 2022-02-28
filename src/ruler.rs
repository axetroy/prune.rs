pub struct Ruler<'a> {
    pub ignore: Vec<String>,
    pub folder: Vec<String>,
    pub file: Vec<String>,
    pub check_only: &'a bool,
}

pub fn parse_rules<'a>() -> Ruler<'a> {
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

#[cfg(test)]
mod tests {
    use crate::ruler::parse_rules;

    #[test]
    fn test_parse_rules() {
        let ruler = parse_rules();

        assert_eq!(ruler.ignore, vec![".git", ".github", ".idea", ".vscode"]);
        assert_eq!(
            ruler.folder,
            vec!["node_modules", "bower_components", ".temp", ".dist"]
        );
        assert_eq!(
            ruler.file,
            vec![
                ".DS_Store",
                ".AppleDouble",
                ".LSOverride",
                "Thumbs.db",
                "Thumbs.db:encryptable",
                "ehthumbs.db",
                "ehthumbs_vista.db"
            ]
        );
        assert_eq!(ruler.check_only, &true);
    }
}
