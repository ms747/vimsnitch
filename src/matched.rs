// TODO(#27) : Abtract matchedLines from main
use std::collections::HashMap;

type File = String;

#[derive(Debug)]
pub struct MatchedLine {
    line_num: usize,
    line: String,
}

impl MatchedLine {
    pub fn new(line_num: usize, line: &str) -> Self {
        MatchedLine {
            line_num,
            line: line.to_string(),
        }
    }

    pub fn get_line(&self) -> &str {
        self.line.as_str()
    }

    pub fn get_line_num(&self) -> usize {
        self.line_num
    }
}

impl std::fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line_num, self.line)
    }
}

pub type Matched = HashMap<File, Vec<MatchedLine>>;
