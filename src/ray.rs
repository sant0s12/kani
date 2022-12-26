use nalgebra as na;
use crate::common::*;

pub struct Ray {
    o: na::Point3<f64>,
    d: na::Vector3<f64>,
    mint: f64,
    maxt: f64,
}

impl Ray {
    pub fn new(o: na::Point3<f64>, d: na::Vector3<f64>, mint: f64, maxt: f64) -> Self {
        Self { d, o, mint, maxt }
    }

    pub fn spawn(o: na::Point3<f64>, d: na::Vector3<f64>) -> Self {
        Self::new(o, d, std::f64::EPSILON, std::f64::MAX)
    }

    pub fn o(&self) -> Point3f {
        self.o
    }

    pub fn d(&self) -> Vector3f {
        self.d
    }

    pub fn mint(&self) -> f64 {
        self.mint
    }

    pub fn maxt(&self) -> f64 {
        self.maxt
    }

    pub fn at(&self, t: f64) -> na::Point3<f64> {
        self.o + t * self.d
    }
}
