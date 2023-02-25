use raytracer::camera::*;
use raytracer::hittable::*;
use raytracer::sphere::*;
use raytracer::vector3::*;

use raytracer::prelude::*;

fn main() {
    let mut scene: HittableList = HittableList::default();

    scene.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    scene.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    render(&camera, &scene);

    println!("Finished!");
}
