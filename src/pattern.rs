// TODO(#38) : Pattern File
use std::path::Path;

#[derive(Debug)]
pub struct Pattern<'a> {
    pub directory: bool,
    pub pattern: glob::Pattern,
    pub root: &'a Path,
    pub negation: bool,
    pub anchored: bool,
}

// plugin/vimsnitch.vim

impl<'a> Pattern<'a> {
    pub fn new(raw_pattern: &str, root: &'a Path) -> Result<Self, String> {
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

    pub fn is_excluded(&self, path: &Path, directory: bool) -> bool {
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

        if !root_path.ends_with('/') && !pattern.starts_with('/') {
            root_path.push('/');
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
