use crate::pattern::Pattern;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Gitignore<'git> {
    pub patterns: Vec<Pattern<'git>>,
    pub root: &'git Path,
    pub included: Vec<PathBuf>,
}

impl<'git> Gitignore<'git> {
    pub fn new(gitignore_path: &'git Path) -> Self {
        let root = gitignore_path.parent().unwrap();
        let patterns = Gitignore::patterns(gitignore_path, root);
        Gitignore {
            patterns,
            root,
            included: Vec::new(),
        }
    }

    fn patterns(path: &'git Path, root: &'git Path) -> Vec<Pattern<'git>> {
        let contents = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error : {}", e);
                std::process::exit(1);
            }
        };
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

    pub fn included_files(&mut self) {
        Gitignore::visit_files(self, self.root);
    }

    pub fn get_files(self) -> Vec<PathBuf> {
        self.included
    }

    fn pattern_found(&self, path: &'git Path, is_dir: bool) -> bool {
        for pat in self.patterns.iter() {
            if pat.is_excluded(&path, is_dir) {
                return true;
            }
        }
        false
    }

    pub fn visit_files(&mut self, path: &Path) {
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
