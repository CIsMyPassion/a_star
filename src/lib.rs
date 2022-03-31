use std::cmp::Ordering;
use std::collections::HashMap;

use rand::distributions::{Bernoulli, Distribution};

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
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

    fn g_cost(point: &Point, other: &Point, g_score_map: &HashMap<Point, usize>) -> usize {

        let x_distance = point.x_distance(other);
        let y_distance = point.y_distance(other);

        let sum = x_distance + y_distance;
        let remainder = sum % 2;

        if remainder == 0 {
            g_score_map[other] + 14
        } else {
            g_score_map[other] + 10
        }
    }

    fn h_cost(point: &Point, goal: &Point) -> usize {
        point.distance(goal)
    }

    fn get_neighbours(&self, center_point: &Point) -> Vec<Point> {
        let mut neighbours = Vec::new();

        let lower_y = if center_point.y > 0 {
            center_point.y - 1
        } else {
            center_point.y
        };

        let lower_x = if center_point.x > 0 {
            center_point.x - 1
        } else {
            center_point.x
        };

        let upper_y = if center_point.y < self.height - 1 {
            center_point.y + 1
        } else {
            center_point.y
        };

        let upper_x = if center_point.x < self.width - 1 {
            center_point.x + 1
        } else {
            center_point.x
        };

        for y in lower_y..upper_y+1 {
            for x in lower_x..upper_x+1 {
                let point = Point::new(x, y);

                if point != *center_point {
                    if self.get_tile(&point) == Tile::Floor {
                        neighbours.push(point);
                    }
                }
            }
        }

        neighbours
    }

    fn reconstruct_path(goal: &Point, came_from_map: &HashMap::<Point, Point>) -> Path {
        let mut steps = Vec::new();
        let mut current = goal;

        steps.push(*current);

        while let Some(next) = came_from_map.get(current) {
            current = next;
            steps.push(*current);

        }

        steps.reverse();
        Path { start: Point::new(0, 0), goal: *goal, steps }
    }

    pub fn find_path(&self, start: &Point, goal: &Point) -> Option<Path> {

        if self.get_tile(start) == Tile::Wall || self.get_tile(goal) == Tile::Wall {
            None

        } else {

            let mut open_set = Vec::<Point>::new();
            let mut came_from_map = HashMap::<Point, Point>::new();
            let mut g_score_map = HashMap::<Point, usize>::new();
            let mut f_score_map = HashMap::<Point, usize>::new();

            open_set.push(*start);
            g_score_map.insert(*start, 0);

            while let Some(point) = open_set.pop() {
                if point == *goal {
                    return Some(Self::reconstruct_path(goal, &came_from_map))
                }

                let neighbours = self.get_neighbours(&point);

                for neighbour in neighbours {

                    let neighbour_g_cost = Self::g_cost(&neighbour, &point, &g_score_map);

                    if !came_from_map.contains_key(&neighbour) {
                        if g_score_map.contains_key(&neighbour) {
                            if g_score_map[&neighbour] > neighbour_g_cost {

                                came_from_map.insert(neighbour, point);
                                g_score_map.insert(neighbour, neighbour_g_cost);
                                f_score_map.insert(neighbour, neighbour_g_cost + Self::h_cost(&neighbour, goal));
                            }

                        } else {

                            came_from_map.insert(neighbour, point);
                            g_score_map.insert(neighbour, neighbour_g_cost);
                            f_score_map.insert(neighbour, neighbour_g_cost + Self::h_cost(&neighbour, goal));

                            open_set.push(neighbour);
                        }
                    }
                }

                open_set.sort_by(|a, b| f_score_map[b].cmp(&f_score_map[a]));
            }

            None
        }
    }
}

pub struct Path {
    start: Point,
    goal: Point,
    steps: Vec<Point>,
}

impl Path {

    pub fn start(&self) -> &Point {
        &self.start
    }

    pub fn goal(&self) -> &Point {
        &self.goal
    }

    pub fn steps(&self) -> &Vec<Point> {
        &self.steps
    }
}
