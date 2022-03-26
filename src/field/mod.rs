use rand::distributions::{Bernoulli, Distribution};

pub enum Tile {
    Floor,
    Wall,
}

pub struct Point {
    x: usize,
    y: usize,
}

impl Point {

    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

pub struct Field {
    width: usize,
    height: usize,
    start: Point,
    goal: Point,
    array: Vec<Tile>,
}

impl Field {

    pub fn new(width: usize, height: usize, start: Point, goal: Point, distribution: f64) -> Self {

        let array = Field::distribution_vec(width, height, distribution);
        Field { width, height, start, goal, array}
    }

    fn distribution_vec(width: usize, height: usize, distribution: f64) -> Vec<Tile> {
        let mut vec: Vec<Tile> = Vec::with_capacity(width * height);

        let distribution = Bernoulli::new(distribution).unwrap();

        for x in 0..width {
            for y in 0..height {
                let sample = distribution.sample(&mut rand::thread_rng());
                if sample {
                    vec.push(Tile::Wall);
                } else {
                    vec.push(Tile::Floor)
                }
            }
        }

        vec
    }

    pub fn widht(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
