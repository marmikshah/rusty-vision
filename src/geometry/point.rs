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
        let x1 = self.x as f32;
        let x2 = point.x as f32;

        let y1 = self.y as f32;
        let y2 = point.y as f32;
        dbg!((x2-x1).powi(2));
        dbg!((y2-y1).powi(2));
        
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    ///
    /// Compute new coordinates for a self when translated to a new shape
    pub fn relocate(&self, original_shape: &Shape, angle: f32) -> Point {
        let width = original_shape.width;
        let height = original_shape.height;

        let (x, y) = (self.x, self.y);

        let (x_new, y_new) = match angle {
            90.0 | -270.0 => (height - 1 - y, x),
            180.0 | -180.0 => (width - 1 - x, height - 1 - y),
            270.0 | -90.0 => (y, width - 1 - x),
            _ => {
                unimplemented!()
            }
        };

        Point { x: x_new, y: y_new }
    }
}

impl Add<Shape> for Point {
    type Output = Point;

    fn add(self, shape: Shape) -> Self::Output {
        let x_new = self.x + shape.width;
        let y_new = self.y + shape.height;

        Self { x: x_new, y: y_new }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
