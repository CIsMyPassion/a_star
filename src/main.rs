use a_star::{Field, Point};
use a_star::visualize::*;

const FIELD_WIDTH: usize = 16;
const FIELD_HEIGHT: usize = 16;
const FIELD_DISTRIBUTION: f64 = 0.4;

fn main() {

    let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT, FIELD_DISTRIBUTION);
    let start = Point::new(0, 0);
    let goal = Point::new(15, 15);
    let mut path = None;

    while path.is_none() {
        field = Field::new(FIELD_WIDTH, FIELD_HEIGHT, FIELD_DISTRIBUTION);
        path = field.find_path(&start, &goal);
    }

    let found = path.unwrap();

    println!("Solution found\nNumber of steps: {}", found.steps().len());
    visualize_scaled_field_with_path(&field, &found, 4, "path.png");
    visualize_scaled_field_with_path_steps(&field, &found, 4, "path.gif")
}
