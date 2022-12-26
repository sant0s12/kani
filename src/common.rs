use image::Rgb;
use nalgebra as na;
use std::ops;

pub(crate) type Point3f = na::Point3<f64>;
pub(crate) type Point2f = na::Point2<f64>;
pub(crate) type Vector3f = na::Vector3<f64>;

pub(crate) struct Color3<T: na::Scalar>(na::Point3<T>);

impl ops::Mul<Color3<f32>> for f64 {
    type Output = Color3<f32>;

    fn mul(self, rhs: Color3<f32>) -> Self::Output {
        Color3(rhs.0 * self as f32)
    }
}

impl ops::Add<Color3<f32>> for Color3<f32> {
    type Output = Self;

    fn add(self, rhs: Color3<f32>) -> Self::Output {
        let vec: Vec<f32> = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| a + b)
            .collect();
        Color3::new(vec[0], vec[1], vec[2])
    }
}

impl<T: na::Scalar> Color3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Color3(na::Point3::<T>::new(x, y, z))
    }
}

impl From<Color3<f32>> for Rgb<f32> {
    fn from(value: Color3<f32>) -> Self {
        Rgb([value.0.x, value.0.y, value.0.z])
    }
}
