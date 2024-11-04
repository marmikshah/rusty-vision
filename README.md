# Rusty Vision

:bangbang: **This is purely experimental and is not intended to be used in production.**  

[![Crates.io](https://img.shields.io/crates/v/rusty-vision.svg)](https://crates.io/crates/rusty-vision)
[![Docs.rs](https://docs.rs/rusty-vision/badge.svg)](https://docs.rs/rusty-vision)
[![CI](https://github.com/marmikshah/rusty-vision/actions/workflows/ci.yml/badge.svg)](https://github.com/marmikshah/rusty-vision/actions/workflows/ci.yml)

## Overview

A basic image processing and manipulation library with the aim to provide OpenCV like functions in Rust. 

:construction: **Since the repo is still in very early phase, please expect breaking changes with new releases.**

## Features

- (WIP) Image Compression & Decompression.
- (WIP) Drawing Shapes.
- (WIP) Image Cropping & Resising.
- (TODO) Background Subtraction.
- (TODO) Optical Flow.
- (Future Plan) HW Accelerated Image Operations.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusty-vision = "0.0.0"
```

or directly run
```bash
cargo add rusty-vision
```


## Usage

Import the core Image module and basic traits

Full code at [01-basic](./examples/01-basic/)

```rust
// Core Image Structure and its traits
use rv::image::Image;
use rv::traits::*;

// Useful structures for geometric operations
use rv::geometry::{Point, Shape};

// Structures and Implenetations for Colors and Channels.
use rv::color::{Color, ColorSpace};

// Image Encoding/Decoding
use rv::codec::Codex;
use rv::io::writer::Writer;
```

Create a blank image with black background.

```rust
let mut image = Image::new(
    Shape {
        width: 1920,
        height: 1080,
        ndim: 3,
    },
    ColorSpace::RGB,
);
```

Draw a rect using the `Drawable` trait.

```rust
let config = RectParams::new(
    Point { x: 10, y: 10 },
    Shape {
        width: 100,
        height: 100,
        ndim: 1,
    },
    Color::new(20, 150, 20, 1.0),
    Some(10),
    Some(0.0),
    None,
);

// Draw
image.draw(&config).unwrap();
```

Save as PNG (Currently only PNG supported)

```rust
// NOTE: `unwrap` can panic
image.write("output.png".to_string(), Codex::PNG).unwrap();
```
