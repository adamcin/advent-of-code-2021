mod common;
/// --- Day 7: The Treachery of Whales ---
///
/// A giant whale has decided your submarine is its next meal, and it's much faster than you are.
/// There's nowhere to run!
///
/// Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise)
/// zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors
/// indicate a massive underground cave system just beyond where they're aiming!
///
/// The crab submarines all need to be aligned before they'll have enough power to blast a large
/// enough hole for your submarine to get through. However, it doesn't look like they'll be aligned
/// before the whale catches you! Maybe you can help?
///
/// There's one major catch - crab submarines can only move horizontally.
///
/// You quickly make a list of the horizontal position of each crab (your puzzle input). Crab
/// submarines have limited fuel, so you need to find a way to make all of their horizontal positions
/// match while requiring them to spend as little fuel as possible.
fn read() -> Vec<usize> {
    let lines: Vec<String> = common::read_test_input("data/day-07/input.txt");
    let crabs: Vec<usize> = lines
        .iter()
        .flat_map(|line| -> Vec<usize> { line.split(',').flat_map(|s| s.parse().ok()).collect() })
        .clone()
        .collect();
    crabs
}

fn align_to(crabs: &Vec<usize>, position: usize, fncost: fn(diff: usize) -> usize) -> usize {
    let mut cost = 0;
    for &crab in crabs {
        if crab < position {
            cost += fncost(position - crab);
        } else {
            cost += fncost(crab - position);
        }
    }
    return cost;
}

/// Determine the horizontal position that the crabs can align to using the least fuel possible. 
/// How much fuel must they spend to align to that position?
#[test]
fn day07part1() {
    let crabs = read();
    assert_eq!(1000, crabs.len(), "expect number of crabs");
    let positions: Vec<usize> = crabs.iter().cloned().collect();
    let costs: Vec<(usize, usize)> = positions
        .iter()
        .map(|&pos| (pos, align_to(&crabs, pos, |d| d)))
        .collect();
    if let Some((init_pos, init_cost)) = costs.last().to_owned() {
        let (_, min_cost): (usize, usize) = costs.iter().fold(
            (*init_pos, *init_cost),
            |(acc_pos, acc_cost), (pos, cost)| {
                if cost < &acc_cost {
                    return (*pos, *cost);
                } else {
                    return (acc_pos, acc_cost);
                }
            },
        );
        assert_eq!(355150, min_cost, "expect cost");
    }
}

/// The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab 
/// engineering?
///
/// As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each 
/// change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first 
/// step costs 1, the second step costs 2, the third step costs 3, and so on.
///
/// As each crab moves, moving further becomes more expensive. This changes the best horizontal 
/// position to align them all on.
#[test]
fn day07part2() {
    let crabs = read();
    assert_eq!(1000, crabs.len(), "expect number of crabs");
    let positions: Vec<usize> = crabs.iter().cloned().collect();
    let costs: Vec<(usize, usize)> = positions
        .iter()
        .map(|&pos| {
            (
                pos,
                align_to(&crabs, pos, |d| (0..=d).fold(0, |a, v| a + v)),
            )
        })
        .collect();
    if let Some((init_pos, init_cost)) = costs.last().to_owned() {
        let (_, min_cost): (usize, usize) = costs.iter().fold(
            (*init_pos, *init_cost),
            |(acc_pos, acc_cost), (pos, cost)| {
                if cost < &acc_cost {
                    return (*pos, *cost);
                } else {
                    return (acc_pos, acc_cost);
                }
            },
        );
        assert_eq!(98368490, min_cost, "expect cost");
    }
}
