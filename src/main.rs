use a_star::{Field, Point};

fn main() {
    let field = Field::new(32, 32, 0.2);
    let start = Point::new(0, 0);
    let goal = Point::new(31, 31);
    field.show();

    match field.find_path(&start, &goal) {
        Some(found) => println!("{:?}\nNumber of steps: {}", found.steps(), found.steps().len()),
        None => println!("No path found"),
    }
}
