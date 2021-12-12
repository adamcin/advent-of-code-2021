mod common;
/// --- Day 2: Dive! ---
///
/// Now, you need to figure out how to pilot this thing.
///
/// It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:
///
/// forward X increases the horizontal position by X units.
/// down X increases the depth by X units.
/// up X decreases the depth by X units.
/// 
/// Note that since you're on a submarine, down and up affect your depth, and so they have the 
/// opposite result of what you might expect.
///
/// The submarine seems to already have a planned course (your puzzle input). You should probably 
/// figure out where it's going.
fn read() -> Vec<SubCommand> {
    let commands: Vec<SubCommand> = common::read_test_input("data/day-02/input.txt")
        .iter()
        .cloned()
        .flat_map(|line| read_command(line))
        .collect();
    commands
}

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

/// Calculate the horizontal position and depth you would have after following the planned course. 
/// What do you get if you multiply your final horizontal position by your final depth?
#[test]
fn day02part1() {
    let commands: Vec<SubCommand> = read();
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

/// Based on your calculations, the planned course doesn't seem to make any sense. You find 
/// the submarine manual and discover that the process is actually slightly more complicated.
///
/// In addition to horizontal position and depth, you'll also need to track a third value, aim, 
/// which also starts at 0. The commands also mean something entirely different than you first 
/// thought:
///
/// down X increases your aim by X units.
/// up X decreases your aim by X units.
/// forward X does two things:
/// It increases your horizontal position by X units.
/// It increases your depth by your aim multiplied by X.
/// 
/// Again note that since you're on a submarine, down and up do the opposite of what you might 
/// expect: "down" means aiming in the positive direction.
#[test]
fn day02part2() {
    let commands: Vec<SubCommand> = read();

    assert_eq!(1000, commands.len(), "expect command vec length");

    let initial_loc: (i64, i64, i64) = (0, 0, 0);

    let (final_x, final_y, _): (i64, i64, i64) = commands.iter().fold(initial_loc, |acc, cmd| {
        let (cur_x, cur_y, cur_aim) = acc;
        return match cmd {
            SubCommand::Up { scale } => (cur_x, cur_y, cur_aim - scale),
            SubCommand::Down { scale } => (cur_x, cur_y, cur_aim + scale),
            SubCommand::Forward { scale } => (cur_x + scale, cur_y + (cur_aim * scale), cur_aim),
        };
    });

    assert_eq!(1282809906, final_x * final_y, "expect command vec length");
}
