mod point_2d {

    #[derive(Debug, PartialEq)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }
}

mod point_3d {
    use super::point_2d::Point as Point2d;

    #[derive(Debug)]
    pub struct Point {
        pub x_y: Point2d,
        pub z: f32,
    }
}

mod util {
    pub use super::point_2d::Point as Point2d;
    pub use super::point_3d::Point as Point3d;

    pub fn _3d_to_2d(point: Point3d) -> Point2d {
        Point2d {
            x: point.x_y.x,
            y: point.x_y.y,
        }
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn test_3d_to_2d() {
        let point = point_3d::Point {
            x_y: point_2d::Point { x: 1.0, y: 2.0 },
            z: 3.0,
        };
        assert_eq!(util::_3d_to_2d(point), point_2d::Point { x: 1.0, y: 2.0 });
    }
}
