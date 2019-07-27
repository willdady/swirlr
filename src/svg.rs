use std::fmt;

#[derive(Debug)]
pub struct Path {
    pub d: PathData,
    fill: String
}

impl Path {
    pub fn new() -> Path {
        Path{
            d: PathData::new(),
            fill: String::from("")
        }
    }

    pub fn set_fill(&mut self, fill: &str) -> &Path {
        self.fill = fill.to_string();
        self
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<path fill=\"{}\" d=\"{}\" />", self.fill, self.d)
    }
}

#[derive(Debug)]
pub struct PathData {
    d: String
}

impl PathData {
    pub fn new() -> PathData {
        PathData {
          d: String::from("")
        }
    }

    fn push_point(&mut self, command: &str, x: f64, y: f64) -> &PathData {
        self.d.push_str(&format!("{}{:.1} {:.1} ", command, x, y));
        self
    }

    pub fn move_to(&mut self, x: f64, y: f64) -> &PathData {
        self.push_point("M", x, y)
    }

    pub fn line_to(&mut self, x: f64, y: f64) -> &PathData {
        self.push_point("L", x, y)
    }

    pub fn close(&mut self) -> &PathData {
        self.d.push_str("Z");
        self
    }
}

impl fmt::Display for PathData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.d)
    }
}
