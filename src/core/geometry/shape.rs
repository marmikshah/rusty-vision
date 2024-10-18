#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    pub width: usize,
    pub height: usize,
    pub ndim: usize,
}

impl Shape {
    pub fn new(width: usize, height: usize, ndim: Option<usize>) -> Shape {
        let ndim = match ndim {
            Some(value) => value,
            None => 1,
        };
        Shape {
            width,
            height,
            ndim,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height * self.ndim
    }
}
