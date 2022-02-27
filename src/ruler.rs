pub struct Ruler<'a> {
    pub ignore: Vec<String>,
    pub folder: Vec<String>,
    pub file: Vec<String>,
    pub check_only: &'a bool,
}

pub fn parse_rules() -> Ruler<'static> {
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
