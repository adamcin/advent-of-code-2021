#[cfg(test)] // The module is only compiled when testing.
mod day01test {
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

        let values: Vec<i64> = reader
            .lines()
            .flat_map(|line| line.map(|str| str.parse()))
            .filter_map(|value| value.ok())
            .collect();

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
        let filename = "data/day-01/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let firsts: Vec<i64> = reader
            .lines()
            .flat_map(|line| line.map(|str| str.parse()))
            .filter_map(|value| value.ok())
            .collect();

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
}
