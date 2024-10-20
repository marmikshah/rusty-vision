#![allow(dead_code)]
use crate::{
    core::{geometry::shape::Shape, image::Image},
    error::Error,
};

#[derive(Default)]
pub struct NearestNeighborParams;

#[derive(Default)]
pub struct BiCubicParams;

#[derive(Debug, Default)]
pub struct BiLinearParams;

pub trait Resizable<T> {
    fn resize(&mut self, shape: Shape) -> Result<(), Error>;
}

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
