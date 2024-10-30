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
fn test_image_ops() {
    let shape = Shape {
        width: 2,
        height: 2,
        ndim: 3,
    };

    let mut image1 = Image::new(shape, ColorSpace::RGB);
    let mut image2 = Image::new(shape, ColorSpace::RGB);

    let color1 = Color::new(20, 0, 0, 1.0);
    let color2 = Color::new(250, 0, 0, 1.0);
    let point = Point::new(0, 0);

    image1.set_pixel(&point, &color1).unwrap();
    image2.set_pixel(&point, &color2).unwrap();

    assert_eq!((image1.clone() + image2.clone())[(0, 0, 0)], 255);
    assert_eq!((image1.clone() - image2.clone())[(0, 0, 0)], 0);
    assert_eq!(
        (image1.clone() & image2.clone())[(0, 0, 0)],
        (20 & 250).clamp(0, 255)
    );
    assert_eq!(
        (image1.clone() | image2.clone())[(0, 0, 0)],
        (20 | 250).clamp(0, 255)
    );
    assert_eq!(
        (image1.clone() ^ image2.clone())[(0, 0, 0)],
        (20 ^ 250).clamp(0, 255)
    );
}
