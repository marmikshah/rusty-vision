#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, point: &Point) -> f32 {
        (self.x as f32 + point.x as f32) / (self.y as f32 + point.y as f32)
    }
}
