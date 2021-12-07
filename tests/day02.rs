#[cfg(test)] // The module is only compiled when testing.
mod day02test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    enum SubCommand {
        Up { scale: i64 },
        Down { scale: i64 },
        Forward { scale: i64 },
    }

    fn parse_command_name(name: &str, scale: i64) -> Option<SubCommand> {
        if "up" == name {
            return Some(SubCommand::Up { scale: scale });
        } else if "down" == name {
            return Some(SubCommand::Down { scale: scale });
        } else if "forward" == name {
            return Some(SubCommand::Forward { scale: scale });
        } else {
            return None;
        }
    }

    fn read_command(line: String) -> Option<SubCommand> {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 2 {
            let cmd = parts[0];
            let sca = parts[1];
            let opt_value: Option<i64> = sca.parse::<i64>().ok();
            return opt_value.and_then(|value| parse_command_name(cmd, value));
        } else {
            return None;
        }
    }

    #[test]
    fn day02part1() {
        let filename = "data/day-02/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let commands: Vec<SubCommand> = reader
            .lines()
            .flat_map(|line_res| line_res.map(|line| read_command(line)))
            .flatten()
            .collect();

        assert_eq!(1000, commands.len(), "expect command vec length");

        let initial_loc: (i64, i64) = (0, 0);

        let (final_x, final_y): (i64, i64) = commands.iter().fold(initial_loc, |acc, cmd| {
            let (cur_x, cur_y) = acc;
            return match cmd {
                SubCommand::Up { scale } => (cur_x, cur_y - scale),
                SubCommand::Down { scale } => (cur_x, cur_y + scale),
                SubCommand::Forward { scale } => (cur_x + scale, cur_y),
            };
        });

        assert_eq!(1480518, final_x * final_y, "expect command vec length");
    }

    #[test]
    fn day02part2() {
        let filename = "data/day-02/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let commands: Vec<SubCommand> = reader
            .lines()
            .flat_map(|line_res| line_res.map(|line| read_command(line)))
            .flatten()
            .collect();

        assert_eq!(1000, commands.len(), "expect command vec length");

        let initial_loc: (i64, i64, i64) = (0, 0, 0);

        let (final_x, final_y, _): (i64, i64, i64) =
            commands.iter().fold(initial_loc, |acc, cmd| {
                let (cur_x, cur_y, cur_aim) = acc;
                return match cmd {
                    SubCommand::Up { scale } => (cur_x, cur_y, cur_aim - scale),
                    SubCommand::Down { scale } => (cur_x, cur_y, cur_aim + scale),
                    SubCommand::Forward { scale } => {
                        (cur_x + scale, cur_y + (cur_aim * scale), cur_aim)
                    }
                };
            });

        assert_eq!(1282809906, final_x * final_y, "expect command vec length");
    }
}