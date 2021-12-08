#[cfg(test)]
mod day07test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    fn day07read(filename: &str) -> Vec<usize> {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().flat_map(|line_r| line_r.ok()).collect();

        let crabs: Vec<usize> = lines
            .iter()
            .flat_map(|line| -> Vec<usize> {
                line.split(',').flat_map(|s| s.parse().ok()).collect()
            })
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

    #[test]
    fn day07part1() {
        let crabs = day07read("data/day-07/input.txt");
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

    #[test]
    fn day07part2() {
        let crabs = day07read("data/day-07/input.txt");
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
}