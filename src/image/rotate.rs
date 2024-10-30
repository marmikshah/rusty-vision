#![allow(unused_variables)]

use super::Image;
use crate::error::Error;
use crate::geometry::{self, Point, Shape};
use crate::traits::*;

impl Rotatable<i32> for Image {
    fn rotate(&mut self, value: i32) -> Result<(), Error> {
        todo!()
    }
}

impl Rotatable<RotationType> for Image {
    fn rotate(&mut self, value: RotationType) -> Result<(), Error> {
        let new_shape = match value {
            RotationType::Clockwise90
            | RotationType::Anticlockwise270
            | RotationType::Clockwise270
            | RotationType::Anticlockwise90 => Shape {
                width: self.height(),
                height: self.width(),
                ndim: self.shape.ndim,
            },
            RotationType::Clockwise180 | RotationType::Anticlockwise180 => self.shape,
            RotationType::Custom(_) => todo!(),
        };

        if new_shape == self.shape {
            for x in 0..self.width() / 2 {
                for y in 0..self.height() {
                    let p1 = Point::new(x, y);
                    let p2 = p1.relocate(&self.shape, value.degree());

                    let idx_a = self.get_index(&p1).unwrap();
                    let idx_b = geometry::get_index_from_point_and_shape(p2, &new_shape).unwrap();

                    for c in 0..self.colorspace.channels() {
                        self.swap(idx_a + c, idx_b + c);
                    }
                }
            }
        } else {
            // TODO: Improve to in-place
            let mut rotated = vec![0; self.size()];
            let ndim = self.shape.ndim;
            for x in 0..self.width() {
                for y in 0..self.height() {
                    let p1 = Point::new(x, y);
                    let p2 = p1.relocate(&self.shape, value.degree());

                    let idx_a = self.get_index(&p1).unwrap();
                    let idx_b = geometry::get_index_from_point_and_shape(p2, &new_shape).unwrap();

                    rotated[idx_b..ndim + idx_b].copy_from_slice(&self.data[idx_a..ndim + idx_a]);
                }
            }

            self.data = rotated;
            self.shape = new_shape;
        }

        Ok(())
    }
}
