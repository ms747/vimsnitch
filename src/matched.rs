// TODO : Abtract matchedLines from main
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
}

impl std::fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:    {}", self.line_num, self.line)
    }
}

pub type Matched = HashMap<File, Vec<MatchedLine>>;
