use log::debug;
use std::ops::Index;

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
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> RectParams {
        let color = Color::new(150, 125, 123, 1.0);

        RectParams {
            x: x,
            y: y,
            width: width,
            height: height,
            color: color,
            border_width: None,
            corner_radius: None,
            fill_color: None,
        }
    }
}

pub struct CircleParams {
    center: usize,
    radius: usize, // Todo: Allow float values
}

pub trait Drawing {
    // TODO: Improve return types
    fn rect(&mut self, params: &RectParams) -> Result<(), ()>;

    fn square(&mut self, params: &RectParams) -> Result<(), ()>;

    fn circle(&mut self, params: &CircleParams) -> Result<(), ()>;
}

impl Drawing for Image {
    fn rect(&mut self, params: &RectParams) -> Result<(), ()> {
        for i in 0..params.width {
            self.set_pixel(params.x + i, params.y, &params.color)?;
            self.set_pixel(params.x, params.y + i, &params.color)?;
            self.set_pixel(params.x + params.width, params.y + i, &params.color)?;
            self.set_pixel(params.x + i, params.y + params.height, &params.color)?;
        }

        Ok(())
    }

    fn square(&mut self, params: &RectParams) -> Result<(), ()> {
        self.rect(params)
    }

    fn circle(&mut self, params: &CircleParams) -> Result<(), ()> {
        todo!()
    }
}
