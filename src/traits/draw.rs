use derive_new::new;

use crate::{
    color::Color,
    error::Error,
    geometry::{Point, Shape},
};

#[derive(Debug, Clone, Copy, new)]
pub struct RectParams {
    pub topleft: Point,
    pub shape: Shape,
    pub color: Color,
    pub border_width: Option<usize>,
    pub corner_radius: Option<f32>,
    pub fill_color: Option<Color>,
}

#[derive(Debug, Clone, Copy, new)]
pub struct CircleParams {
    pub center: Point,
    pub radius: usize,
    pub color: Color,
    pub fill_color: Option<Color>,
}

#[derive(Debug, Clone, Copy)]
pub struct TriangleParams;

pub trait Drawable<T> {
    fn draw(&mut self, params: &T) -> Result<(), Error>;
}
