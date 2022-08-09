use bitcoin::ecc::Point;

fn main() {
    let p1 = Point::new(-1., -1., 5., 7.);
    let p2 = Point::new(-1., -1., 5., 7.);
    let inf = Point::new_inf(5., 7.);

    println!("{:?}", &p1+&p2);
}
