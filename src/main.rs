// mod vecmath:vec;

mod math;

use math::vec::{angle_between, fma, gram_schmidt, lerp, Vec3};

fn main() {
    println!("Hello, world!");

    let t1 = Vec3::new(1.0, 2.0, 5.0);
    let t2 = Vec3::new(1.0, 2.0, 4.0);

    let t3 = t1 + t2;
    println!("{:?}", t3);

    let t4 = t1 * 3.;

    println!("{:?}", t4);

    println!("{}", t1 == t2);

    let t5 = Vec3::new(-1., -1., -6.5);
    let t6 = Vec3::new(-1., 5.2, -6.5);

    println!("{:?}", t5.abs());
    println!("{:?}", t5.ceil());
    println!("{:?}", t6.floor());

    let a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(4., 5., 6.);

    println!("{:?}", lerp(0.5, a, b));
    println!("{:?}", fma(a, b, b));

    let t7 = Vec3::new(3., 4., 0.);
    println!("{}", t7.length());

    let t8 = Vec3::new(4., 5., 0.);
    // let t8 = Vec3::new(1., 2., 3.);
    println!("{:?}", t8.norm());

    println!("{}", angle_between(t7, t8));

    println!("{}", t8.dot(gram_schmidt(t7, t8)) == 0.);
}
