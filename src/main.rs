use a_star::{Field, Point};
use a_star::visualize::*;

const FIELD_WIDTH: usize = 64;
const FIELD_HEIGHT: usize = 64;
const FIELD_DISTRIBUTION: f64 = 0.6;

fn main() {

    let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT, FIELD_DISTRIBUTION);
    let start = Point::new(0, 0);
    let goal = Point::new(63, 63);
    let mut path = None;

    let mut counter = 1;

    while path.is_none() {
        println!("Create field {}", counter);
        field = Field::new(FIELD_WIDTH, FIELD_HEIGHT, FIELD_DISTRIBUTION);
        path = field.find_path(&start, &goal);
        counter += 1;
    }

    let found = path.unwrap();

    println!("Solution found\nNumber of steps: {}", found.steps().len());
    visualize_scaled_field_with_path(&field, &found, 2, "path.png");
    visualize_scaled_field_with_path_steps(&field, &found, 2, "path.gif")
}
