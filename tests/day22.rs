mod common;

fn read() -> Vec<Cuboid> {
    parse_input(common::read_test_input("data/day-22/input.txt"))
}

fn read_test() -> Vec<Cuboid> {
    parse_input(common::read_test_input("data/day-22/input_test.txt"))
}

fn read_test2() -> Vec<Cuboid> {
    parse_input(common::read_test_input("data/day-22/input_test2.txt"))
}

fn parse_input(lines: Vec<String>) -> Vec<Cuboid> {
    lines
        .iter()
        .map(|line| Cuboid::parse(line).unwrap())
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum State {
    Off,
    On,
}

use State::*;
impl State {
    fn of(val: &str) -> Option<Self> {
        match val {
            "off" => Some(Off),
            "on" => Some(On),
            _ => None,
        }
    }
}

type Coord = (isize, isize, isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cuboid {
    state: State,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

use std::cmp::{max, min};
impl Cuboid {
    fn parse(line: &String) -> Option<Self> {
        if let Some((state_str, coords_str)) = line.split_once(" ") {
            if let Some(state) = State::of(state_str) {
                let coords: Vec<isize> = coords_str
                    .split(",")
                    .flat_map(|part| part[2..].split(".."))
                    .flat_map(|i_str| i_str.parse().ok())
                    .collect();
                return Some(Self {
                    state: state,
                    x_min: min(coords[0], coords[1]),
                    x_max: max(coords[0], coords[1]),
                    y_min: min(coords[2], coords[3]),
                    y_max: max(coords[2], coords[3]),
                    z_min: min(coords[4], coords[5]),
                    z_max: max(coords[4], coords[5]),
                });
            }
        }
        None
    }

    fn volume(&self) -> isize {
        (1 + self.x_max - self.x_min)
            * (1 + self.y_max - self.y_min)
            * (1 + self.z_max - self.z_min)
    }

    fn betw(bound1: &isize, value: &isize, bound2: &isize) -> bool {
        min(bound1, bound2) <= value && value <= max(bound1, bound2)
    }

    fn dim_overlaps(bounds1: &(isize, isize), bounds2: &(isize, isize)) -> bool {
        let ((lmin, lmax), (rmin, rmax)) = ((bounds1), (bounds2));
        Self::betw(lmin, rmin, lmax)
            || Self::betw(lmin, rmax, lmax)
            || Self::betw(rmin, lmin, rmax)
            || Self::betw(rmin, lmax, rmax)
    }

    fn x_bounds(&self) -> (isize, isize) {
        (self.x_min, self.x_max)
    }

    fn y_bounds(&self) -> (isize, isize) {
        (self.y_min, self.y_max)
    }

    fn z_bounds(&self) -> (isize, isize) {
        (self.z_min, self.z_max)
    }

    fn overlaps(&self, other: &Self) -> bool {
        Self::dim_overlaps(&self.x_bounds(), &other.x_bounds())
            && Self::dim_overlaps(&self.y_bounds(), &other.y_bounds())
            && Self::dim_overlaps(&self.z_bounds(), &other.z_bounds())
    }

    fn subsets(&self, other: &Self) -> bool {
        Self::betw(&other.x_min, &self.x_min, &other.x_max)
            && Self::betw(&other.x_min, &self.x_max, &other.x_max)
            && Self::betw(&other.y_min, &self.y_min, &other.y_max)
            && Self::betw(&other.y_min, &self.y_max, &other.y_max)
            && Self::betw(&other.z_min, &self.z_min, &other.z_max)
            && Self::betw(&other.z_min, &self.z_max, &other.z_max)
    }

    fn new_with_x(&self, bounds: (isize, isize)) -> Self {
        let (x_min, x_max) = bounds;
        Self {
            state: self.state,
            x_min: x_min,
            x_max: x_max,
            y_min: self.y_min,
            y_max: self.y_max,
            z_min: self.z_min,
            z_max: self.z_max,
        }
    }

    fn new_with_y(&self, bounds: (isize, isize)) -> Self {
        let (y_min, y_max) = bounds;
        Self {
            state: self.state,
            x_min: self.x_min,
            x_max: self.x_max,
            y_min: y_min,
            y_max: y_max,
            z_min: self.z_min,
            z_max: self.z_max,
        }
    }

    fn new_with_z(&self, bounds: (isize, isize)) -> Self {
        let (z_min, z_max) = bounds;
        Self {
            state: self.state,
            x_min: self.x_min,
            x_max: self.x_max,
            y_min: self.y_min,
            y_max: self.y_max,
            z_min: z_min,
            z_max: z_max,
        }
    }

    fn split_x_min(&self, other: &Self) -> Vec<Self> {
        if &self.x_min < &other.x_min {
            if &self.x_max < &other.x_min {
                vec![*self]
            } else {
                vec![
                    vec![self.new_with_x((self.x_min, other.x_min - 1))],
                    self.new_with_x((other.x_min, self.x_max)).split_x_max(other),
                ].concat()
            }
        } else {
            self.split_x_max(other)
        }
    }

    fn split_x_max(&self, other: &Self) -> Vec<Self> {
        if &self.x_max > &other.x_max {
            if &self.x_min > &other.x_max {
                vec![*self]
            } else {
                vec![
                    vec![self.new_with_x((other.x_max + 1, self.x_max))],
                    self.new_with_x((self.x_min, other.x_max)).split_y_min(other),
                ].concat()
            }
        } else {
            self.split_y_min(other)
        }
    }

    fn split_y_min(&self, other: &Self) -> Vec<Self> {
        if &self.y_min < &other.y_min {
            if &self.y_max < &other.y_min {
                vec![*self]
            } else {
                vec![
                    vec![self.new_with_y((self.y_min, other.y_min - 1))],
                    self.new_with_y((other.y_min, self.y_max)).split_y_max(other),
                ].concat()
            }
        } else {
            self.split_y_max(other)
        }
    }

    fn split_y_max(&self, other: &Self) -> Vec<Self> {
        if &self.y_max > &other.y_max {
            if &self.y_min > &other.y_max {
                vec![*self]
            } else {
                vec![
                    vec![self.new_with_y((other.y_max + 1, self.y_max))],
                    self.new_with_y((self.y_min, other.y_max)).split_z_min(other),
                ].concat()     
            }
        } else {
            self.split_z_min(other)
        }
    }

    fn split_z_min(&self, other: &Self) -> Vec<Self> {
        if &self.z_min < &other.z_min {
            if &self.z_max < &other.z_min {
                vec![*self]
            } else {
                vec![
                    vec![self.new_with_z((self.z_min, other.z_min - 1))],
                    self.new_with_z((other.z_min, self.z_max)).split_z_max(other),
                ].concat()
            }
        } else {
            self.split_z_max(other)
        }
    }

    fn split_z_max(&self, other: &Self) -> Vec<Self> {
        if &self.z_max > &other.z_max {
            if &self.z_min > &other.z_max {
                vec![*self]
            } else {
                vec![self.new_with_z((other.z_max + 1, self.z_max))]
            }
        } else {
            vec![]
        }
    }

    fn fragment(&self, step: &Self) -> Vec<Self> {
        self.split_x_min(step)
    }
}

struct Reactor {
    on: Vec<Cuboid>,
}

impl Reactor {
    fn new() -> Self {
        Self { on: Vec::new() }
    }

    fn step(&self, cuboid: &Cuboid) -> Self {
        let mut new_on: Vec<Cuboid> = Vec::new();
        for c in self.on.iter() {
            new_on.extend(c.fragment(cuboid).iter());
        }
        if cuboid.state == On {
            new_on.push(*cuboid);
        }
        Self { on: new_on }
    }

    fn count_on(&self) -> usize {
        self.on.iter().fold(0, |a, v| a + v.volume() as usize)
    }
}

#[test]
fn day22_test_read() {
    let steps = read_test();
    assert_eq!(22, steps.len());

    // on x=-20..26,y=-36..17,z=-47..7
    assert_eq!(
        Cuboid {
            state: On,
            x_min: -20,
            x_max: 26,
            y_min: -36,
            y_max: 17,
            z_min: -47,
            z_max: 7
        },
        steps[0]
    );
}

#[test]
fn day22pre_part1() {
    let reactor = read_test()
        .iter()
        .take(20)
        .fold(Reactor::new(), |reactor, s| reactor.step(s));
    assert_eq!(590784, reactor.count_on());
}

#[test]
fn day22part1() {
    let reactor = read()
        .iter()
        .take(20)
        .fold(Reactor::new(), |reactor, s| reactor.step(s));
    assert_eq!(561032, reactor.count_on());
}

#[test]
fn day22pre_part2() {
    let reactor = read_test2()
        .iter()
        .fold(Reactor::new(), |reactor, s| reactor.step(s));
    assert_eq!(2758514936282235, reactor.count_on());
}

#[test]
fn day22part2() {
    let reactor = read()
        .iter()
        .fold(Reactor::new(), |reactor, s| reactor.step(s));
    assert_eq!(1322825263376414, reactor.count_on());
}
