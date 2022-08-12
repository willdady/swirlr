use std::fmt;

#[derive(Debug)]
pub struct Path {
    pub d: PathData,
    fill: String,
}

impl Path {
    pub fn new() -> Path {
        Path {
            d: PathData::new(),
            fill: String::from(""),
        }
    }

    pub fn set_fill(mut self, fill: &str) -> Path {
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
    d: String,
}

impl PathData {
    pub fn new() -> PathData {
        PathData {
            d: String::from(""),
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

pub struct Svg {
    pub width: u32,
    pub height: u32,
    pub children: Vec<Box<dyn fmt::Display>>,
}

impl Svg {
    pub fn new(width: u32, height: u32) -> Svg {
        Svg {
            width,
            height,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Box<dyn fmt::Display>) {
        self.children.push(child);
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\">",
            w = self.width,
            h = self.height
        )?;
        for child in &self.children {
            write!(f, "{}", *child)?;
        }
        write!(f, "</svg>")?;
        Ok(())
    }
}

pub struct Rect {
    pub width: u32,
    pub height: u32,
    pub fill: Option<String>,
}

impl Rect {
    pub fn new(width: u32, height: u32) -> Rect {
        Rect {
            width,
            height,
            fill: None,
        }
    }

    pub fn set_fill(mut self, fill: Option<String>) -> Rect {
        self.fill = fill;
        self
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<rect width=\"{w}\" height=\"{h}\" {fill} />",
            w = self.width,
            h = self.height,
            fill = if let Some(f) = &self.fill {
                format!("fill=\"{}\"", f)
            } else {
                "".to_string()
            }
        )?;
        Ok(())
    }
}
