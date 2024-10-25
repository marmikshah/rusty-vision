#![allow(dead_code)]
use crate::{error::Error, geometry::Shape};

#[derive(Default)]
pub struct NearestNeighborParams;

#[derive(Default)]
pub struct BiCubicParams;

#[derive(Debug, Default)]
pub struct BiLinearParams;

pub trait Resizable<T> {
    fn resize(&mut self, shape: Shape) -> Result<(), Error>;
}
