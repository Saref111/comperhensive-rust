// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::ops::Add;
use core::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    // add fields
    x: f64,
    y: f64
}

impl Point {
    // add methods
    fn new(x: i32, y: i32) -> Self {
        Self {x: x as f64, y: y as f64}
    }

    /// d=√((x2 – x1)² + (y2 – y1)²)
    fn dist(&self, p: Self) -> f64 {
        ((p.x - self.x).powi(2) + (p.y - self.y).powi(2)).sqrt()
    }

    fn magnitude(&self) -> f64 {
        self.dist(Self {x: 0.0, y: 0.0})
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, p: Self) -> Self {
        Self {
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        }
    }
}

pub struct Polygon {
    // add fields
    points: Vec<Point>
}

impl Polygon {
    // add methods
    fn new() -> Self {
        Polygon { 
            points: Vec::new()
         }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    fn iter(&self) -> Iter<'_, Point> {
        self.points.iter()
    }

    fn left_most_point(&self) -> Option<Point> {
        if self.points.is_empty() {
            None
        } else {
            let mut the_leftest_point = self.points[0];

            for point in &self.points[1..] {
                if point.dist(the_leftest_point) < 0.0 {
                    the_leftest_point = *point
                }
            }

            Some(the_leftest_point)
        }
    }

    pub fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut result = 0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            result += last_point.dist(*point);
            last_point = *point;
        }
        result += last_point.dist(self.points[0]);
        result
        // Alternatively, Iterator::zip() lets us iterate over the points as pairs
        // but we need to pair each point with the next one, and the last point
        // with the first point. The zip() iterator is finished as soon as one of 
        // the source iterators is finished, a neat trick is to combine Iterator::cycle
        // with Iterator::skip to create the second iterator for the zip and using map 
        // and sum to calculate the total length.
    }
}

pub struct Circle {
    // add fields
    center: Point,
    radius: f64
}

impl Circle {
    fn new(center: Point, rad: i32) -> Self {
        Self {
            center,
            radius: rad as f64
        }
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn dist(&self, other: Circle) -> f64 {
        self.center.dist(other.center)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(s: &Self) -> f64 {
        match s {
            Self::Circle(circle) => {
                circle.circumference()
            },
            Self::Polygon(poly) => {
                poly.length()
            }
        }
    }
}

impl From<Polygon> for Shape {
    fn from(p: Polygon) -> Self {
        Self::Polygon(p)
    }
}

impl From<Circle> for Shape {
    fn from(c: Circle) -> Self {
        Self::Circle(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude().into()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
