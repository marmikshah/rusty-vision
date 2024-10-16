use log::debug;
use std::ops::{Add, Index};

use crate::{
    core::{color::Color, image::Image},
    error::Error,
};

pub struct RectParams {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: Color,
    border_width: Option<usize>,
    corner_radius: Option<f32>,
    fill_color: Option<Color>,
}

impl RectParams {
    pub fn new(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        border_width: Option<usize>,
    ) -> RectParams {
        let color = Color::new(150, 125, 123, 1.0);

        RectParams {
            x,
            y,
            width,
            height,
            color,
            border_width,
            corner_radius: None,
            fill_color: None,
        }
    }
}

pub struct CircleParams {
    center: usize,
    radius: usize, // TODO: Allow float values
}

pub trait Drawing {
    // TODO: Improve return types
    fn rect(&mut self, params: &RectParams) -> Result<(), Error>;

    fn square(&mut self, params: &RectParams) -> Result<(), Error>;

    fn circle(&mut self, params: &CircleParams) -> Result<(), Error>;
}

impl Drawing for Image {
    fn rect(&mut self, params: &RectParams) -> Result<(), Error> {
        let border_width = match params.border_width {
            Some(value) => value,
            None => 0,
        };
        for i in 0..params.width + border_width {
            let range = match params.border_width {
                Some(value) => (-((value / 2) as i32), ((value / 2) + 1) as i32),
                None => (0, 1),
            };

            dbg!(range);
            for k in range.0..range.1 {
                // Top Edge
                self.set_pixel(
                    params.x + i - (border_width / 2) as usize,
                    (params.y as i32 + k) as usize,
                    &params.color,
                )?;
                // Left Edge
                self.set_pixel(
                    (params.x as i32 + k) as usize,
                    params.y + i - (border_width / 2 as usize),
                    &params.color,
                )?;
                // Right Edge
                self.set_pixel(
                    ((params.x + params.width) as i32 - k) as usize,
                    params.y + i - (border_width / 2) as usize,
                    &params.color,
                )?;
                // Bottom Edge
                self.set_pixel(
                    params.x + i - (border_width / 2) as usize,
                    ((params.y + params.height) as i32 - k) as usize,
                    &params.color,
                )?;
            }
        }

        Ok(())
    }

    fn square(&mut self, params: &RectParams) -> Result<(), Error> {
        self.rect(params)
    }

    fn circle(&mut self, params: &CircleParams) -> Result<(), Error> {
        todo!()
    }
}
