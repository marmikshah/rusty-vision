use rusty_vision as rv;
use rv::geometry::Point;
use rv::geometry::Shape;

#[test]
fn test_shape_size() {
    let shape = Shape::new(100, 100, Some(3));
    assert_eq!(shape.size(), 100 * 100 * 3);
}

#[test]
fn test_point_add_point() {
    let p1 = Point::new(10, 10);
    let p2 = Point::new(10, 10);

    assert_eq!(p1 + p2, Point { x: 20, y: 20 });
}

#[test]
fn test_point_add_shape() {
    let p1 = Point::new(10, 10);
    let p2 = Shape::new(10, 10, None);

    assert_eq!(p1 + p2, Point { x: 20, y: 20 });
}

#[test]
fn test_distance_between_points() {
    let p1 = Point::new(10, 10);
    let p2 = Point::new(20, 20);

    assert_eq!((p1.distance(&p2) * 1000.0).round(), 14142.0);
}
