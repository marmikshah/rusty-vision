/**
 * Implementation of common traits for Image type
 */
use super::Image;
use crate::core::geometry::{Point, Shape};

use crate::core::traits::*;
use crate::error::Error;

/* ------------------ DRAWING TRAIT ------------------ */
impl Drawable<RectParams> for Image {
    ///
    /// Draw a rectangle on the Image
    ///
    fn draw(&mut self, params: &RectParams) -> Result<(), Error> {
        let border_width = params.border_width.unwrap_or_default();

        match params.fill_color {
            Some(color) => {
                let top_left = params.topleft;
                let bottom_right = params.topleft + params.shape;

                for i in top_left.x..bottom_right.x + 1 {
                    for j in top_left.y..bottom_right.y + 1 {
                        let point = Point::new(i, j);
                        self.set_pixel(&point, &color)?;
                    }
                }
            }
            None => {
                dbg!("Fill not enabled");
            }
        };

        for i in 0..params.shape.width + border_width {
            let range = match params.border_width {
                Some(value) => (-((value / 2) as i32), ((value / 2) + 1) as i32),
                None => (0, 1),
            };

            dbg!(range);
            for k in range.0..range.1 {
                let x_top_left = params.topleft.x;
                let y_top_left = params.topleft.y;

                // Top Edge
                let top = Point::new(
                    x_top_left + i - (border_width / 2),
                    (y_top_left as i32 + k) as usize,
                );
                dbg!(top);
                self.set_pixel(&top, &params.color)?;

                // Left Side
                let left = Point::new(
                    (x_top_left as i32 + k) as usize,
                    y_top_left + i - (border_width / 2),
                );
                dbg!(left);
                self.set_pixel(&left, &params.color)?;

                // Right Edge
                let right = Point::new(
                    ((x_top_left + params.shape.width) as i32 - k) as usize,
                    y_top_left + i - (border_width / 2),
                );
                dbg!(right);
                self.set_pixel(&right, &params.color)?;

                // Bottom Edge
                let bottom = Point::new(
                    x_top_left + i - (border_width / 2),
                    ((y_top_left + params.shape.height) as i32 - k) as usize,
                );
                dbg!(bottom);
                self.set_pixel(&bottom, &params.color)?;
            }
        }

        Ok(())
    }
}

impl Drawable<CircleParams> for Image {
    ///
    /// Draw a circle on the Image using the Midpoint Circle Algorithm.
    ///
    fn draw(&mut self, params: &CircleParams) -> Result<(), Error> {
        println!("Drawing circle {params:?}");

        let mut x = params.radius;
        let mut y = 0;

        let mut p: f32 = 1.0 - params.radius as f32;

        while x >= y {
            dbg!(x, y, p);

            let symmetric = [
                [
                    Point::new(params.center.x - x, params.center.y + y),
                    Point::new(params.center.x + x, params.center.y + y),
                ],
                [
                    Point::new(params.center.x - x, params.center.y - y),
                    Point::new(params.center.x + x, params.center.y - y),
                ],
                [
                    Point::new(params.center.x - y, params.center.y + x),
                    Point::new(params.center.x + y, params.center.y + x),
                ],
                [
                    Point::new(params.center.x - y, params.center.y - x),
                    Point::new(params.center.x + y, params.center.y - x),
                ],
            ];

            for pair in symmetric {
                if let Some(color) = params.fill_color {
                    let mut start = pair[0].clone();
                    let end = pair[1].clone();
                    while start.x < end.x - 1 {
                        start.x += 1;
                        dbg!(start);
                        self.set_pixel(&start, &color)?;
                    }
                }

                self.set_pixel(&pair[0], &params.color)?;
                self.set_pixel(&pair[1], &params.color)?;
            }

            y += 1;

            if p <= 0.0 {
                p = p + 2.0 * y as f32 + 1.0;
            } else {
                x -= 1;
                p = p + 2.0 * y as f32 - 2.0 * x as f32 + 1.0;
            }
        }

        Ok(())
    }
}
/* ------------------ ------- ----- ------------------ */

/* ------------------ RESIZE TRAIT ------------------ */
impl Resizable<NearestNeighborParams> for Image {
    fn resize(&mut self, shape: Shape) -> Result<(), Error> {
        todo!()
    }
}

impl Resizable<BiCubicParams> for Image {
    fn resize(&mut self, shape: Shape) -> Result<(), Error> {
        todo!()
    }
}

impl Resizable<BiLinearParams> for Image {
    fn resize(&mut self, shape: Shape) -> Result<(), Error> {
        todo!()
    }
}
/* ------------------ ------ ----- ------------------ */

/* ------------------ ROTATION TRAIT ------------------ */

impl Rotatable<i32> for Image {
    fn rotate(&mut self, value: i32) -> Result<(), Error> {
        // Custom rotation by degrees.
        todo!()
    }
}

impl Rotatable<RotationType> for Image {
    fn rotate(&mut self, value: RotationType) -> Result<(), Error> {
        match value {
            RotationType::CLOCKWISE90 => todo!(),
            RotationType::CLOCKWISE180 | RotationType::ANTICLICKWISE180 => {
                for y in 0..self.height() {
                    for x in 0..self.width() / 2 {
                        for c in 0..self.colorspace.channels() {
                            let idx_a = self.get_index_from_xy(x, y).unwrap() + c;
                            let idx_b =
                                self.get_index_from_xy(self.width() - x - 1, y).unwrap() + c;

                            self.swap(idx_a, idx_b);
                        }
                    }
                }
                Ok(())
            }
            RotationType::CLOCKWISE270 => todo!(),
            RotationType::ANTICLOCKWISE90 => todo!(),
            RotationType::ANTICLICKWISE270 => todo!(),
        }
    }
}
/* ------------------ -------- ----- ------------------ */
