#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn dist(&self, point: &Point) -> f64 {
        ((point.x - self.x).powi(2) + (point.y - self.y).powi(2)).sqrt()
    }

    pub fn angle(&self, point: &Point) -> f64 {
        let delta_x = point.x - self.x;
        let delta_y = point.y - self.y;
        delta_y.atan2(delta_x)
    }
}

impl From<(u32, u32)> for Point {
    fn from(t: (u32, u32)) -> Self {
        Point::new(t.0.into(), t.1.into())
    }
}

impl From<(f64, f64)> for Point {
    fn from(t: (f64, f64)) -> Self {
        Point::new(t.0, t.1)
    }
}
