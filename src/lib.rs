use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use rand::distributions::{Bernoulli, Distribution};

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {

    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn distance(&self, other: &Point) -> usize {

        let x_distance = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let y_distance = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        let sum = x_distance * x_distance + y_distance + y_distance;

        (sum as f64).sqrt() as usize

    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.x.cmp(&self.x)
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub struct Field {
    width: usize,
    height: usize,
    array: Vec<Tile>,
}

impl Field {

    pub fn new(width: usize, height: usize, distribution: f64) -> Self {

        let array = Field::distribution_vec(width, height, distribution);

        Field { width, height, array}
    }

    fn distribution_vec(width: usize, height: usize, distribution: f64) -> Vec<Tile> {
        let mut vec: Vec<Tile> = Vec::with_capacity(width * height);

        let distribution = Bernoulli::new(distribution).unwrap();

        for _x in 0..width {
            for _y in 0..height {
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn show(&self) {

        let black_box: char = char::from_u32(0x25A0).unwrap();
        let white_box: char = char::from_u32(0x25A1).unwrap();
        let newline: char = char::from_u32(0x000A).unwrap();

        //let mut output_line: Vec<char> = vec![white_box; (self.width + 1) * self.height];
        let mut output_vec: Vec<char> = Vec::with_capacity((self.width + 1) * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.array[x + y * self.width];
                output_vec.push(match value {
                    Tile::Floor => white_box,
                    Tile::Wall => black_box,
                });
            }

            output_vec.push(newline);
        }

        output_vec.pop();

        let output: String = output_vec.iter().collect();
        println!("{}", output);

    }

    pub fn get_tile(&self, point: &Point) -> Tile {
        self.array[point.x + point.y * self.width]
    }

    fn h_cost(&self, point: &Point, goal: &Point) -> usize {
        point.distance(goal)
    }

    fn get_neighbours(&self, position: &Point, goal: &Point) -> Vec<PathPoint> {
        //TODO: collect neighbours aka floor tiles
        Vec::new()
    }

    pub fn find_path(&self, start: &Point, goal: &Point) -> Option<Path> {

        if self.get_tile(start) == Tile::Wall || self.get_tile(goal) == Tile::Wall {
            None

        } else {

            let mut open_set = BinaryHeap::<PathPoint>::new();
            let mut came_from_map = HashMap::<PathPoint, PathPoint>::new();
            let mut g_score_map = HashMap::<PathPoint, usize>::new();
            let mut f_score_map = HashMap::<PathPoint, usize>::new();

            let mut start_path_point = PathPoint::new(*start, 0, self.h_cost(start, goal));

            open_set.push(start_path_point);

            while let Some(PathPoint { position, g_cost, h_cost }) = open_set.pop() {
                if position == *goal {
                    //TODO: reconstruct path
                }

                let neighbours = self.get_neighbours(&position, goal);

                for neighbour in neighbours {

                }
            }

            None
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct PathPoint {
    position: Point,
    g_cost: usize,
    h_cost: usize,
}

impl PathPoint {

    pub fn new(position: Point, g_cost: usize, h_cost: usize) -> Self {
        PathPoint { position, g_cost, h_cost }
    }

    pub fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

impl Ord for PathPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&self.f_cost())
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PathPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Path {
    start: Point,
    goal: Point,
    steps: Vec<PathPoint>,
}

impl Path {

    pub fn new(start: Point, goal: Point) -> Self {
        Path { start, goal, steps: Vec::new() }
    }

    pub fn start(&self) -> &Point {
        &self.start
    }

    pub fn goal(&self) -> &Point {
        &self.goal
    }

    pub fn steps(&self) -> &Vec<PathPoint> {
        &self.steps
    }
}
