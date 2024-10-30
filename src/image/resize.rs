#![allow(unused_variables)]

use super::Image;
use crate::error::Error;
use crate::geometry::Shape;
use crate::traits::*;

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
