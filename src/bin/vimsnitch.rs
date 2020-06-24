use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::path::Path;

use vimsnitch::gitignore::Gitignore;
use vimsnitch::gitissue::GitIssue;
use vimsnitch::matched::{Matched, MatchedLine};

fn main() -> Result<(), http_types::Error> {
    let mut storage: Matched = HashMap::new();

    let issues = GitIssue::new(
        "ms747",
        "vimsnitch.git",
        "126562439d17dc58ab483485ff006b4af0ef07d3",
    );

    let mut current_path = current_dir().unwrap();
    current_path.push(".gitignore");
    // TODO(#28) : Remove all unwraps
    let current_path = Path::new(current_path.to_str().unwrap());

    let mut ignore = Gitignore::new(current_path);
    ignore.included_files();

    let todo_regex = Regex::new(r"//\s*TODO\s*:\s*(.*)").unwrap();

    let mut found = false;

    for file in ignore.get_files().iter() {
        let file = file.to_str().unwrap();
        let contents = std::fs::read_to_string(file).expect("Unable to open file");
        let mut line_matches: Vec<MatchedLine> = vec![];
        for (i, line) in contents.lines().enumerate() {
            let line = line.trim();
            let matches = todo_regex.captures(line);
            if matches.iter().len() > 0 {
                let matched = match &matches {
                    Some(data) => data.get(1).unwrap().as_str().trim(),
                    _ => panic!("Regular Expression Failed"),
                };
                line_matches.push(MatchedLine::new(i + 1, matched));
                found = true;
            }
        }

        if found {
            storage.insert(file.to_string(), line_matches);
            found = false;
        }
    }

    let mut todos = vec![];
    for (file, matches) in storage.iter() {
        println!("{}", &file[1..]);
        for capture in matches.iter() {
            todos.push(capture.get_line());
            println!("{}", capture);
        }
    }

    // TODO(#29) : Better variable naming

    let new_issues = issues
        .create_many(&todos)?
        .iter()
        .map(|val| val.number as usize)
        .collect::<Vec<usize>>();

    let mut new_issues_index = 0;
    for (file, patterns) in storage.iter() {
        let contents = std::fs::read_to_string(&file).expect("Unable to Read File");
        let mut new_contents = String::new();
        let mut pattern_index: usize = 0;
        for (i, line) in contents.lines().enumerate() {
            if pattern_index > patterns.len() - 1 {
                pattern_index -= 1;
            }
            if patterns[pattern_index].get_line_num() == i + 1 {
                let editied = todo_regex.replace(line, |capture: &regex::Captures| {
                    format!(
                        "// TODO(#{}) : {}",
                        new_issues[new_issues_index], &capture[1]
                    )
                });
                new_contents.push_str(&format!("{}\n", editied));
                pattern_index += 1;
                new_issues_index += 1;
                continue;
            } else {
                new_contents.push_str(&format!("{}\n", line));
            }
        }
        std::fs::write(file, new_contents.as_str()).expect("Unable to Write File");
    }
    Ok(())
}
