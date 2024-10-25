#![allow(dead_code)]
use crate::error::Error;

pub enum RotationType {
    Clockwise90,
    Clockwise180,
    Clockwise270,
    Anticlockwise90,
    Anticlockwise180,
    Anticlockwise270,
    Custom(f32),
}

impl RotationType {
    pub fn degree(&self) -> f32 {
        match self {
            RotationType::Clockwise90 => 90.0,
            RotationType::Clockwise180 => 180.0,
            RotationType::Clockwise270 => 270.0,
            RotationType::Anticlockwise90 => -90.0,
            RotationType::Anticlockwise180 => -180.0,
            RotationType::Anticlockwise270 => -270.0,
            RotationType::Custom(value) => *value,
        }
    }
}

pub trait Rotatable<T> {
    fn rotate(&mut self, value: T) -> Result<(), Error>;
}
