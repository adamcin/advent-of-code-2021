pub fn read_test_input(filename: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().flat_map(|line_r| line_r.ok()).collect();
}