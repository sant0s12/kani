use camera::*;
use common::*;
use image::{ImageBuffer, Rgb, Rgb32FImage};
use nalgebra as na;
use ray::Ray;

pub mod camera;
pub mod common;
pub mod frame;
pub mod ray;

fn ray_color(ray: &Ray) -> Color3<f32> {
    let t = 0.5 * (ray.d().y + 1.0);
    println!("{}", t);
    (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let camera = PerspectiveCamera::new(Point3f::new(0.0, 0.0, 0.0), 2.0 * aspect_ratio, 2.0, 1.0);

    let mut img: Rgb32FImage = ImageBuffer::new((512.0 * aspect_ratio) as u32, 512);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let u = x as f64 / (img.width() - 1) as f64;
            let v = y as f64 / (img.height() - 1) as f64;

            let ray = camera.cast_ray(Point2f::new(u, v));
            img.put_pixel(x, img.height() - 1 - y, ray_color(&ray).into());
        }
    }

    img.save("test.exr").unwrap();
}
