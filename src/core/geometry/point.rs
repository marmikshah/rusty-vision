use std::ops::Add;

use derive_new::new;

use super::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq, new)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn distance(&self, point: &Point) -> f32 {
        (self.x as f32 + point.x as f32) / (self.y as f32 + point.y as f32)
    }
}

impl Add<Shape> for Point {
    type Output = Point;

    fn add(self, shape: Shape) -> Self::Output {
        let x_new = self.x + shape.width;
        let y_new = self.y + shape.height;

        Point { x: x_new, y: y_new }
    }
}
