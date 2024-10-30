use rusty_vision as rv;
use rv::{
    color::{Color, ColorSpace},
    geometry::Point,
    geometry::Shape,
    image::Image,
};


#[test]
fn test_image_create() {
    let image = Image::new(
        Shape {
            width: 2,
            height: 2,
            ndim: 3,
        },
        ColorSpace::RGB,
    );
    assert_eq!(image.size(), 2 * 2 * 3)
}

#[test]
fn test_image_add() {
    let shape = Shape {
        width: 2,
        height: 2,
        ndim: 3,
    };

    let mut image1 = Image::new(shape, ColorSpace::RGB);
    let mut image2 = Image::new(shape, ColorSpace::RGB);

    let color = Color::new(20, 0, 0, 1.0);
    let point = Point::new(0, 0);

    image1.set_pixel(&point.clone(), &color.clone()).unwrap();

    image2.set_pixel(&point.clone(), &color.clone()).unwrap();
    image1 = image1 + image2;

    assert_eq!(image1[(0, 0, 0)], 40);
}

#[test]
fn test_image_add_out_of_range() {
    let shape = Shape {
        width: 2,
        height: 2,
        ndim: 3,
    };

    let mut image1 = Image::new(shape, ColorSpace::RGB);
    let mut image2 = Image::new(shape, ColorSpace::RGB);

    let color = Color::new(200, 0, 0, 1.0);
    let point = Point::new(0, 0);

    image1.set_pixel(&point.clone(), &color.clone()).unwrap();

    image2.set_pixel(&point.clone(), &color.clone()).unwrap();
    image1 = image1 + image2;

    assert_eq!(image1[(0, 0, 0)], 255);
}

#[test]
fn test_bitwise_and() {
    let shape = Shape {
        width: 2,
        height: 2,
        ndim: 3,
    };

    let mut image1 = Image::new(shape, ColorSpace::RGB);
    let mut image2 = Image::new(shape, ColorSpace::RGB);

    let color = Color::new(200, 0, 0, 1.0);
    let point = Point::new(0, 0);

    image1.set_pixel(&point.clone(), &color.clone()).unwrap();

    image2.set_pixel(&point.clone(), &color.clone()).unwrap();
    image1 = image1 & image2;

    assert_eq!(image1[(0, 0, 0)], 0);
}
