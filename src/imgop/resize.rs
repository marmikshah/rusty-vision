#![allow(dead_code)]
use crate::{core::image::Image, error::Error};

#[derive(Debug)]
pub enum ResizeMethod {
    NearestNeighbors,
    Bilinear,
    Bicubic,
}

pub trait Resize {
    fn resize(
        &mut self,
        new_width: u32,
        new_height: u32,
        method: ResizeMethod,
    ) -> Result<(), Error>;
}

impl Resize for Image {
    fn resize(
        &mut self,
        new_width: u32,
        new_height: u32,
        method: ResizeMethod,
    ) -> Result<(), Error> {
        match method {
            ResizeMethod::NearestNeighbors => resize_nearest_neighbors(self, new_width, new_height),
            ResizeMethod::Bilinear => resize_bilinear(self, new_width, new_height),
            ResizeMethod::Bicubic => resize_bicubic(self, new_width, new_height),
        }
        Ok(())
    }
}

fn resize_nearest_neighbors(image: &mut Image, new_width: u32, new_height: u32) {
    todo!()
}

fn resize_bilinear(image: &mut Image, new_width: u32, new_height: u32) {
    todo!()
}

fn resize_bicubic(image: &mut Image, new_width: u32, new_height: u32) {
    todo!()
}
