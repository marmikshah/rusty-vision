# Rusty Vision

:bangbang: **NOTE: This is purely experimental and is not intended to be used in production.**  

[![Crates.io](https://img.shields.io/crates/v/rusty-vision.svg)](https://crates.io/crates/rusty-vision)
[![Docs.rs](https://docs.rs/rusty-vision/badge.svg)](https://docs.rs/rusty-vision)
[![Build Status](https://img.shields.io/github/actions/workflow/status/marmikshah/rusty-vision/publish.yml)](https://github.com/marmikshah/rusty-vision/actions)

## Overview

A basic image processing and manipulation library with the aim to provide OpenCV like functions in Rust. 

:construction: **NOTE: Since the repo is still in very early phase, please expect breaking changes with new releases.**

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

Full code at [draw-rect.rs](./examples/draw-rect.rs). 

```rust

use rusty_vision as rv;
use rv::types::Image;
use rv::core::traits::*;

use core::geometry::{Point, Shape};

// Structures and Implenetations for Colors and Channels.
use rv::core::{ColorSpace, Color};

// Image Encoding/Decoding
use rv::codec::Codex;
use rv::io::writer::Writer;
```

Create a blank image with black background.

```rust
let shape = Shape::new(1920, 1080, Some(3));
let mut image = Image::new(shape, ColorSpace::RGB);
```

Draw a rect using the `Drawable` trait.

```rust
// `ndim` can be None
let rect = Shape::new(100, 100, None);
let topleft = Point::new(10, 10);

let config = RectParams::new(
    topleft,
    rect,
    Color::new(20, 150, 20, 1.0),
    Some(10),
    Some(0.0), 
    None,
);

// NOTE: `unwrap` can panic
image.draw(&config).unwrap();
```

Save as PNG (Currently only PNG supported)

```rust
// NOTE: `unwrap` can panic
image.write("output.png".to_string(), Codex::PNG).unwrap();
```
