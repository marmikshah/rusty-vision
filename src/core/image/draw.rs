use crate::{
    core::{color::Color, geometry::point::Point, geometry::shape::Shape, image::Image},
    error::Error,
};
use log::debug;

#[derive(Debug, Clone, Copy)]
pub struct RectParams {
    topleft: Point,
    shape: Shape,
    color: Color,
    border_width: Option<usize>,
    corner_radius: Option<f32>,
    fill_color: Option<Color>,
}

impl RectParams {
    pub fn new(topleft: Point, shape: Shape, border_width: Option<usize>) -> RectParams {
        let color = Color::new(150, 125, 123, 1.0);
        let fill_color = Color::new(25, 125, 25, 1.0);

        RectParams {
            topleft,
            shape,
            color,
            border_width,
            corner_radius: None,
            fill_color: Some(fill_color),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircleParams {
    center: usize,
    radius: usize, // TODO: Allow float values
}

pub trait Drawable<T> {
    fn draw(&mut self, params: &T) -> Result<(), Error>;
}

impl Drawable<RectParams> for Image {
    fn draw(&mut self, params: &RectParams) -> Result<(), Error> {
        let border_width = match params.border_width {
            Some(value) => value,
            None => 0,
        };

        dbg!(params.shape);
        // TODO: Implement fill
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
                    x_top_left + i - (border_width / 2) as usize,
                    (y_top_left as i32 + k) as usize,
                );
                dbg!(top);
                self.set_pixel(&top, &params.color)?;

                // Left Side
                let left = Point::new(
                    (x_top_left as i32 + k) as usize,
                    y_top_left + i - (border_width / 2 as usize),
                );
                dbg!(left);
                self.set_pixel(&left, &params.color)?;

                // Right Edge
                let right = Point::new(
                    ((x_top_left + params.shape.width) as i32 - k) as usize,
                    y_top_left + i - (border_width / 2) as usize,
                );
                dbg!(right);
                self.set_pixel(&right, &params.color)?;

                // Bottom Edge
                let bottom = Point::new(
                    x_top_left + i - (border_width / 2) as usize,
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
    fn draw(&mut self, params: &CircleParams) -> Result<(), Error> {
        debug!("Drawing circle {params:?}");
        todo!()
    }
}
