mod common;
/// --- Day 1: Sonar Sweep ---
///
/// You're minding your own business on a ship at sea when the overboard alarm goes off! 
/// You rush to see if you can help. Apparently, one of the Elves tripped and accidentally 
/// sent the sleigh keys flying into the ocean!
///
/// Before you know it, you're inside a submarine the Elves keep ready for situations like this. 
/// It's covered in Christmas lights (because of course it is), and it even has an experimental 
/// antenna that should be able to track the keys if you can boost its signal strength high enough; 
/// there's a little meter that indicates the antenna's signal strength by displaying 0-50 stars.
///
/// Your instincts tell you that in order to save Christmas, you'll need to get all fifty stars 
/// by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent 
/// calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. 
/// Good luck!
///
/// As the submarine drops below the surface of the ocean, it automatically performs a sonar sweep 
/// of the nearby sea floor. On a small screen, the sonar sweep report (your puzzle input) appears: 
/// each line is a measurement of the sea floor depth as the sweep looks further and further away 
/// from the submarine.
fn read() -> Vec<i64> {
    let lines: Vec<String> = common::read_test_input("data/day-01/input.txt");
    let values: Vec<i64> = lines
        .iter()
        .flat_map(|line| line.parse().ok())
        .clone()
        .collect();
    values
}

/// The first order of business is to figure out how quickly the depth increases, just so you 
/// know what you're dealing with - you never know if the keys will get carried into deeper water 
/// by an ocean current or a fish or something.
///
/// To do this, count the number of times a depth measurement increases from the previous measurement. 
/// (There is no measurement before the first measurement.) 
/// 
/// How many measurements are larger than the previous measurement?
#[test]
fn day01part1() {
    let values: Vec<i64> = read();

    assert_eq!(2000, values.len(), "expect vec len");

    let increases: Vec<i64> = values[1..]
        .iter()
        .zip(values.iter())
        .map(|(next, value)| next - value)
        .filter(|diff| diff > &0)
        .collect();

    assert_eq!(1557, increases.len(), "expect increases len");
}

/// Considering every single measurement isn't as useful as you expected: there's just too much 
/// noise in the data.
///
/// Consider sums of a three-measurement sliding window. 
/// 
/// How many sums are larger than the previous sum?
#[test]
fn day01part2() {
    let firsts: Vec<i64> = read();

    assert_eq!(2000, firsts.len(), "expect vec len");

    // zip3 iterating from thirds vector, then sum
    let sums: Vec<i64> = firsts[2..] // thirds
        .iter()
        .zip(firsts[1..].iter()) // seconds
        .zip(firsts.iter()) // firsts
        .map(|((third, second), first)| third + second + first)
        .collect();

    let increases: Vec<i64> = sums[1..]
        .iter()
        .zip(sums.iter())
        .map(|(next, value)| next - value)
        .filter(|diff| diff > &0)
        .collect();

    assert_eq!(1608, increases.len(), "expect increases len");
}
