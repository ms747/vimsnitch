#![allow(dead_code)]
use std::fs;
use std::path::{Path, PathBuf};

fn visit_all_files(dir: &Path, pattern: &Pattern) {
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if pattern.is_excluded(path.as_path(), true) {
                            continue;
                        }
                        visit_all_files(&path, pattern);
                    } else {
                        if pattern.is_excluded(path.as_path(), false) {
                            continue;
                        }
                        println!("{:?}", path);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Pattern<'a> {
    directory: bool,
    pattern: glob::Pattern,
    root: &'a Path,
    negation: bool,
    anchored: bool,
}

impl<'a> Pattern<'a> {
    fn new(raw_pattern: &str, root: &'a Path) -> Result<Self, String> {
        let mut pattern = raw_pattern.to_string();
        let directory = pattern.ends_with('/');

        if directory {
            pattern.pop();
        }

        let anchored = pattern.contains('/');
        let negation = pattern.starts_with('!');

        if negation {
            pattern.remove(0);
            pattern = pattern.trim_start().to_string();
        }

        let abs_pattern = Pattern::abs_pattern(&pattern, root, anchored);
        let pattern = glob::Pattern::new(&abs_pattern).expect("Unable to parse pattern");

        Ok(Pattern {
            pattern,
            anchored,
            negation,
            directory,
            root,
        })
    }

    fn is_excluded(&self, path: &Path, directory: bool) -> bool {
        if self.directory && !directory {
            return self.negation;
        }

        self.negation ^ self.pattern.matches_path_with(&path, self.match_options())
    }

    fn abs_pattern(pattern: &str, root: &Path, anchored: bool) -> String {
        if anchored {
            Pattern::anchored_pattern(pattern, root)
        } else if !pattern.starts_with('*') {
            Pattern::unanchored_pattern(pattern)
        } else {
            pattern.to_string()
        }
    }

    fn unanchored_pattern(pattern: &str) -> String {
        let root_path = "*".to_string();
        root_path + pattern
    }

    fn anchored_pattern(pattern: &str, root: &Path) -> String {
        let mut root_path = root.to_str().unwrap().to_string();

        if root_path.ends_with('/') {
            root_path.pop();
        }
        root_path + pattern
    }

    fn match_options(&self) -> glob::MatchOptions {
        glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: self.anchored,
            require_literal_leading_dot: false,
        }
    }
}

#[derive(Debug)]
struct Gitignore<'a> {
    patterns: Vec<Pattern<'a>>,
    root: &'a Path,
    included: Vec<PathBuf>,
}

impl<'a> Gitignore<'a> {
    fn new(gitignore_path: &'a Path) -> Self {
        let root = gitignore_path.parent().unwrap();
        let patterns = Gitignore::patterns(gitignore_path, root);
        Gitignore {
            patterns,
            root,
            included: Vec::new(),
        }
    }

    fn patterns(path: &'a Path, root: &'a Path) -> Vec<Pattern<'a>> {
        let contents = fs::read_to_string(path).expect("Unable to Read Line");
        contents
            .lines()
            .filter_map(|line| {
                if !line.trim().is_empty() && !line.starts_with('#') {
                    Pattern::new(line, root).ok()
                } else {
                    None
                }
            })
            .collect()
    }

    fn included_files(&mut self) {
        Gitignore::visit_files(self, self.root);
    }

    fn get_files(self) -> Vec<PathBuf> {
        self.included
    }

    fn pattern_found(&self, path: &'a Path, is_dir: bool) -> bool {
        self.patterns.iter().fold(false, |acc, pattern: &Pattern| {
            if pattern.is_excluded(&path, is_dir) {
                true
            } else {
                acc
            }
        })
    }

    fn visit_files(&mut self, path: &Path) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    // Ignore git files
                    if path.ends_with(".git") || path.ends_with(".gitignore") {
                        continue;
                    }

                    if path.is_dir() {
                        let found: bool = self.pattern_found(&path, true);

                        if found {
                            continue;
                        }

                        Gitignore::visit_files(self, &path);
                    } else {
                        // Collect ignored files
                        if !self.pattern_found(&path, false) {
                            self.included.push(path);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut current_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    current_dir.push_str("/.gitignore");
    let current_path = Path::new(&current_dir);
    let mut gitignore = Gitignore::new(current_path);
    gitignore.included_files();
    for file in gitignore.get_files().iter() {
        println!("{}", file.to_str().unwrap());
    }
}
