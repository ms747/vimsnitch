use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::path::Path;

use vimsnitch::gitignore::Gitignore;

type File = String;

#[derive(Debug)]
struct MatchedLine {
    line_num: usize,
    line: String,
}

impl MatchedLine {
    fn new(line_num: usize, line: &str) -> Self {
        MatchedLine {
            line_num,
            line: line.to_string(),
        }
    }
}

impl std::fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:    {}", self.line_num, self.line)
    }
}

type Matched = HashMap<File, Vec<MatchedLine>>;

fn main() {
    // TODO : Todo 1
    let mut storage: Matched = HashMap::new();

    let mut current_path = current_dir().unwrap();
    current_path.push(".gitignore");
    let current_path = Path::new(current_path.to_str().unwrap());

    let mut ignore = Gitignore::new(current_path);
    ignore.included_files();

    // TODO : Todo 2
    let todo_regex = Regex::new(r"^//\s+TODO\s+:\s+(.+)").unwrap();

    let mut found = false;

    for file in ignore.get_files().iter() {
        let file = file.to_str().unwrap();
        let contents = std::fs::read_to_string(file).expect("Unable to open file");
        let mut line_matches: Vec<MatchedLine> = vec![];
        for (i, line) in contents.lines().enumerate() {
            let line = line.trim();
            let matches = todo_regex.captures(line);

            if matches.iter().len() > 0 {
                line_matches.push(MatchedLine::new(i, line));
                found = true;
            }
        }
        if found {
            storage.insert(file.to_string(), line_matches);
            found = false;
        }
    }

    for (file, matches) in storage.iter() {
        println!("{}", &file[1..]);
        for capture in matches.iter() {
            println!("{}", capture);
        }
    }
}
