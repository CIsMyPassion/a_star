use a_star::{Field, Point};

fn main() {
    let field = Field::new(32, 32, 0.3);
    let start = Point::new(0, 0);
    let end = Point::new(31, 31);
    field.show();
}
