mod common;

fn read() -> Vec<String> {
    let values = common::read_test_input("data/day-11/input.txt");
    values
}

#[test]
fn day11part1() {
    let input = read();
    assert_eq!(10, input.len(), "expect lines of input");
}