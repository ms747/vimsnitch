// TODO(#40) : Abtract matchedLines from main
pub type File = String;

#[derive(Debug, Clone)]
pub struct MatchedLine {
    pub line_num: usize,
    pub line: String,
}

impl MatchedLine {
    pub fn new(line_num: usize, line: &str) -> Self {
        MatchedLine {
            line_num,
            line: line.to_string(),
        }
    }

    pub fn get_line(&self) -> String {
        self.line.clone()
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
