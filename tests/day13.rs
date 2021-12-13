mod common;

/// --- Day 13: Transparent Origami ---
///
/// You reach another volcanically active part of the cave. It would be nice if you could do
/// some kind of thermal imaging so you could tell ahead of time which caves are too hot to
/// safely enter.
///
/// Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it,
/// you are greeted with:
///
/// Congratulations on your purchase! To activate this infrared thermal imaging
/// camera system, please enter the code found on page 1 of the manual.
///
/// Apparently, the Elves have never used this feature. To your surprise, you manage to find
/// the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper!
/// The transparent paper is marked with random dots and includes instructions on how to fold it
/// up (your puzzle input).
fn read() -> AllInput {
    let mut coords: Vec<Coord> = Vec::new();
    let mut folds: Vec<FoldInst> = Vec::new();
    let input = common::read_test_input("data/day-13/input.txt");
    let mut now_folds = false;
    for line in input {
        if line.is_empty() {
            now_folds = true;
            continue;
        }
        if now_folds {
            parse_fold(&mut folds, &line);
        } else {
            parse_coord(&mut coords, &line);
        }
    }
    return (coords, folds);
}

fn parse_coord(coords: &mut Vec<Coord>, line: &String) {
    let parts: Vec<usize> = line.split(',').flat_map(|part| part.parse().ok()).collect();
    coords.push((parts[0], parts[1]));
}

use FoldInst::{Left, Up};
fn parse_fold(folds: &mut Vec<FoldInst>, line: &String) {
    let parts: Vec<&str> = line.split('=').collect();
    let inst: String = parts[0].to_owned();
    let value: usize = parts[1].parse().ok().expect("expect parse usize");
    if let Some(dir) = inst.chars().last() {
        if dir == 'x' {
            folds.push(Left { value: value });
        } else if dir == 'y' {
            folds.push(Up { value: value });
        }
    }
}

#[derive(Debug)]
enum FoldInst {
    Up { value: usize },
    Left { value: usize },
}

/// x,y
type Coord = (usize, usize);
type AllInput = (Vec<Coord>, Vec<FoldInst>);

use std::collections::HashSet;
fn fold(coords: &Vec<Coord>, inst: &FoldInst) -> Vec<Coord> {
    let mut indexes: Vec<usize> = Vec::new();
    let folded: Vec<(usize, Coord)> = match *inst {
        Left { value } => coords
            .iter()
            .enumerate()
            .filter(|(_, (x, _))| x > &value)
            .map(|(i, (x, y))| (i, (value - (x - value), *y)))
            .collect(),
        Up { value } => coords
            .iter()
            .enumerate()
            .filter(|(_, (_, y))| y > &value)
            .map(|(i, (x, y))| (i, (*x, value - (y - value))))
            .collect(),
    };
    let mut new_coords: HashSet<Coord> = HashSet::new();
    for (i, coord) in folded {
        indexes.push(i);
        new_coords.insert(coord);
    }
    indexes.sort();
    indexes.reverse();
    let mut remove_folded: Vec<Coord> = coords.iter().cloned().collect();
    for i in indexes {
        remove_folded.remove(i);
    }
    for coord in remove_folded {
        new_coords.insert(coord);
    }
    return new_coords.iter().cloned().collect();
}

/// The transparent paper is pretty big, so for now, focus on just completing the first fold.
/// After the first fold in the example above, 17 dots are visible - dots that end up overlapping
/// after the fold is completed count as a single dot.
///
/// How many dots are visible after completing just the first fold instruction on your transparent paper?
#[test]
fn day13part1() {
    let (coords, folds) = read();
    assert_eq!(1125, coords.len(), "expect n coordinates");
    assert_eq!(12, folds.len(), "expect n fold instructions");

    let folded = fold(&coords, &folds[0]);
    assert_ne!(1484, folded.len(), "not right");
    assert_eq!(942, folded.len(), "expect n visible dots");
}
fn max(left: usize, right: usize) -> usize {
    if left > right {
        left
    } else {
        right
    }
}

#[test]
fn day13part2() {
    let (coords, folds) = read();
    let folded = folds.iter().fold(coords, |acc, inst| fold(&acc, &inst));

    let max_x = folded.iter().fold(0, |ax, (x, _)| max(ax, *x));
    let max_y = folded.iter().fold(0, |ay, (_, y)| max(ay, *y));

    println!("{:?}", (max_x, max_y));

    let mut points: HashSet<Coord> = HashSet::new();
    for coord in folded {
        points.insert(coord);
    }

    println!("OUTPUT");
    println!("");
    for y in 0..=max_y {
        for x in 0..=max_x {
            let point = (x, y);
            if points.contains(&point) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    println!("");
    println!("FINISHED...visual confirmation required: JZGUAPRB");
}
