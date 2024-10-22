#![allow(dead_code)]
use crate::error::Error;

pub enum RotationType {
    CLOCKWISE90,
    CLOCKWISE180,
    CLOCKWISE270,
    ANTICLOCKWISE90,
    ANTICLICKWISE180,
    ANTICLICKWISE270,
}

pub trait Rotatable<T> {
    fn rotate(&mut self, value: T) -> Result<(), Error>;
}
