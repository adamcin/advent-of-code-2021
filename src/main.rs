fn main() {
    println!("Hello, world!");
}

#[cfg(test)] // The module is only compiled when testing.
mod test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    // This function is a test function. It will be executed and
    // the test will succeed if the function exits cleanly.
    #[test]
    fn day01part1() {
        let filename = "data/day-01/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let values = reader
            .lines()
            .flat_map(|line| line.map(|str| str.parse::<i64>()))
            .filter_map(|value| value.ok())
            .collect::<Vec<_>>();

        assert_eq!(2000, values.len(), "expect vec len");

        let increases = values[1..]
            .iter()
            //.copied()
            .zip(values.iter())
            .map(|(next, value)| next - value)
            .filter(|diff| diff > &0)
            .collect::<Vec<_>>();

        assert_eq!(1557, increases.len(), "expect increases len");
    }

    #[test]
    fn day01part2() {
        let filename = "data/day-01/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let firsts = reader
            .lines()
            .flat_map(|line| line.map(|str| str.parse::<i64>()))
            .filter_map(|value| value.ok())
            .collect::<Vec<_>>();

        assert_eq!(2000, firsts.len(), "expect vec len");

        let sums: Vec<i64> = firsts[2..]
            .iter()
            .zip(firsts[1..].iter())
            .map(|(third, second)| third + second)
            .zip(firsts.iter())
            .map(|(left, first)| left + first)
            .collect();

        let increases = sums[1..]
            .iter()
            .zip(sums.iter())
            .map(|(next, value)| next - value)
            .filter(|diff| diff > &0)
            .collect::<Vec<_>>();

        assert_eq!(1608, increases.len(), "expect increases len");
    }
}
