mod common;

fn read() -> Vec<i64> {
    let lines: Vec<String> = common::read_test_input("data/day-01/input.txt");
    let values: Vec<i64> = lines
        .iter()
        .flat_map(|line| line.parse().ok())
        .clone()
        .collect();
    values
}

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
