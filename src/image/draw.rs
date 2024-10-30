use super::Image;
use crate::error::Error;
use crate::geometry::Point;
use crate::traits::*;

impl Drawable<RectParams> for Image {
    fn draw(&mut self, params: &RectParams) -> Result<(), Error> {
        let border_width = params.border_width.unwrap_or_default();

        if let Some(color) = params.fill_color {
            let top_left = params.topleft;
            let bottom_right = params.topleft + params.shape;

            for i in top_left.x..bottom_right.x + 1 {
                for j in top_left.y..bottom_right.y + 1 {
                    let point = Point::new(i, j);
                    self.set_pixel(&point, &color)?;
                }
            }
        }

        for i in 0..params.shape.width + border_width {
            let range = match params.border_width {
                Some(value) => (-((value / 2) as i32), ((value / 2) + 1) as i32),
                None => (0, 1),
            };

            for k in range.0..range.1 {
                let x_top_left = params.topleft.x;
                let y_top_left = params.topleft.y;

                // Top Edge
                let top = Point::new(
                    x_top_left + i - (border_width / 2),
                    (y_top_left as i32 + k) as usize,
                );
                self.set_pixel(&top, &params.color)?;

                // Left Side
                let left = Point::new(
                    (x_top_left as i32 + k) as usize,
                    y_top_left + i - (border_width / 2),
                );
                self.set_pixel(&left, &params.color)?;

                // Right Edge
                let right = Point::new(
                    ((x_top_left + params.shape.width) as i32 - k) as usize,
                    y_top_left + i - (border_width / 2),
                );
                self.set_pixel(&right, &params.color)?;

                // Bottom Edge
                let bottom = Point::new(
                    x_top_left + i - (border_width / 2),
                    ((y_top_left + params.shape.height) as i32 - k) as usize,
                );
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
                    let mut start = pair[0];
                    let end = pair[1];
                    while start.x < end.x - 1 {
                        start.x += 1;
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
