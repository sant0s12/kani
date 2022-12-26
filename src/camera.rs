use crate::{common::*, ray::*};

pub trait Camera {
    fn cast_ray(&self, uv: Point2f) -> Ray;
}

pub struct PerspectiveCamera {
    origin: Point3f,
    horizontal: Vector3f,
    vertical: Vector3f,
    lower_left_corner: Point3f,
    focal_length: f64,
}

impl PerspectiveCamera {
    pub fn new(origin: Point3f, width: f64, height: f64, focal_length: f64) -> Self {
        let horizontal = Vector3f::new(width, 0.0, 0.0);
        let vertical = Vector3f::new(0.0, height, 0.0);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vector3f::new(0.0, 0.0, focal_length),
            focal_length,
        }
    }

    pub fn width(&self) -> f64 {
        self.horizontal.norm()
    }

    pub fn height(&self) -> f64 {
        self.vertical.norm()
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn origin(&self) -> Point3f {
        self.origin
    }
}

impl Camera for PerspectiveCamera {
    fn cast_ray(&self, uv: Point2f) -> Ray {
        let mut d = self.lower_left_corner + uv.x * self.horizontal + uv.y * self.vertical - self.origin;
        d.normalize_mut();

        Ray::new(
            self.origin,
            d,
            self.focal_length,
            std::f64::MAX,
        )
    }
}
