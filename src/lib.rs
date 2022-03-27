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

    pub fn x_distance(&self, other: &Point) -> usize {
        if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        }
    }

    pub fn y_distance(&self, other: &Point) -> usize {
        if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        }
    }

    pub fn distance(&self, other: &Point) -> usize {

        let x_distance = self.x_distance(other);
        let y_distance = self.y_distance(other);

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

    fn g_cost(path_point: &PathPoint, other: &Point) -> usize {
        let x_distance = path_point.position().x_distance(other);
        let y_distance = path_point.position().y_distance(other);

        let sum = x_distance + y_distance;
        let remainder = sum % 2;

        if remainder == 0 {
            14
        } else {
            10
        }
    }

    fn h_cost(point: &Point, goal: &Point) -> usize {
        point.distance(goal)
    }

    fn get_neighbours(&self, path_point: &PathPoint, goal: &Point) -> Vec<PathPoint> {
        //TODO: collect neighbours aka floor tiles
        let mut neighbours = Vec::new();

        for y in path_point.position().y-1..path_point.position().y+1 {
            for x in path_point.position().x-1..path_point.position().x+1 {
                let point = Point::new(x, y);

                if point != path_point.position {
                    if self.get_tile(&point) == Tile::Floor {
                        let new_g_cost = Self::g_cost(path_point, &point);
                        let new_h_cost = Self::h_cost(&point, goal);
                        neighbours.push(PathPoint::new(point, new_g_cost, new_h_cost));
                    }
                }
            }
        }

        neighbours
    }

    pub fn find_path(&self, start: &Point, goal: &Point) -> Option<Path> {

        if self.get_tile(start) == Tile::Wall || self.get_tile(goal) == Tile::Wall {
            None

        } else {

            let mut open_set = BinaryHeap::<PathPoint>::new();
            let mut came_from_map = HashMap::<Point, Point>::new();
            let mut g_score_map = HashMap::<Point, usize>::new();
            let mut f_score_map = HashMap::<Point, usize>::new();

            let mut start_path_point = PathPoint::new(*start, 0, Self::h_cost(start, goal));

            open_set.push(start_path_point);

            while let Some(path_point) = open_set.pop() {
                if path_point.position() == *goal {
                    //TODO: reconstruct path
                }

                let neighbours = self.get_neighbours(&path_point, goal);

                for neighbour in neighbours {

                    if !came_from_map.contains_key(&neighbour.position()) {
                        if neighbour.g_cost() < g_score_map[&neighbour.position()] {
                            came_from_map.insert(neighbour.position(), path_point.position());
                            g_score_map.insert(neighbour.position(), neighbour.g_cost());
                            f_score_map.insert(neighbour.position(), neighbour.f_cost());

                            if open_set.
                        }
                    }
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

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn g_cost(&self) -> usize {
        self.g_cost
    }

    pub fn h_cost(&self) -> usize {
        self.h_cost
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
