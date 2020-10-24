use raytracer::create_ppm;
use raytracer::vec3::Vec3;

fn main() {
    // create_ppm::test_image();
    let vec = Vec3::vec_with_xyz(1.0, 1.0, 1.0);
    let vec1 = Vec3::default();
    println!("{:?}", vec);
    println!("{:?}", vec1);
}
