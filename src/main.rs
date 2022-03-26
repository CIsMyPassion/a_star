use a_star::field::{Point, Field};

fn main() {
    let s = Point::new(0, 0);
    let g = Point::new(31, 31);
    let f = Field::new(32, 32, s, g, 0.3);
    println!("Hello, world!");
    println!("Width: {}, Height: {}", f.widht(), f.height())
}
