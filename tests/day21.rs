mod common;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Player {
    turns: usize, // n of turns completed before this turn
    index: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize, turns: usize) -> Self {
        assert!(pos >= 1 && pos <= 10);
        Self {
            turns: turns,
            index: pos - 1,
            score: 0,
        }
    }

    fn play(&self, spaces: usize) -> Self {
        let new_index = (self.index + spaces) % 10;
        Self {
            turns: self.turns + 1,
            index: new_index,
            score: self.score + new_index + 1,
        }
    }

    fn num_rolls(&self) -> usize {
        self.turns * 3
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Die {
    Determ { seed: usize, side: usize },
    Hyper,
}

use Die::*;

impl Die {

    fn roll(&self, player: &Player) -> Vec<(Die, Player)> {
        match self {
            Determ { seed, side } => vec![(
                Determ {
                    seed: (seed + 3) % side,
                    side: *side,
                },
                player.play(
                    [seed % side, (seed + 1) % side, (seed + 2) % side]
                        .iter()
                        .map(|v| v + 1)
                        .fold(0, |a, v| a + v),
                ),
            )],
            Hyper => vec![],
        }
    }
}

fn read() -> (Player, Player) {
    (Player::new(9, 0), Player::new(3, 0))
}

fn read_test() -> (Player, Player) {
    (Player::new(4, 0), Player::new(8, 0))
}

fn play_until_score(win_score: usize, player1: &Player, player2: &Player) -> (Player, Player) {
    let mut die = Determ { seed: 0, side: 100 };
    let mut player1 = *player1;
    let mut player2 = *player2;

    for turn in 0.. {
        if turn % 2 == 0 {
            let (next_die, player) = *die.roll(&player1).first().unwrap();
            player1 = player;
            die = next_die;
        } else {
            let (next_die, player) = *die.roll(&player2).first().unwrap();
            player2 = player;
            die = next_die;
        }
        if player1.score >= win_score || player2.score >= win_score {
            break;
        }
    }

    (player1, player2)
}

#[test]
fn day21pre_part1() {
    let (og_player1, og_player2) = read_test();
    let (player1, player2) = play_until_score(1000, &og_player1, &og_player2);

    assert_eq!(
        739785,
        std::cmp::min(player1.score, player2.score) * (player1.num_rolls() + player2.num_rolls())
    );
}

#[test]
fn day21part1() {
    let (og_player1, og_player2) = read();
    let (player1, player2) = play_until_score(1000, &og_player1, &og_player2);

    assert_eq!(
        1073709,
        std::cmp::min(player1.score, player2.score) * (player1.num_rolls() + player2.num_rolls())
    );
}

#[test]
fn day21pre_part2() {
    let p1_winner_universes = 444356092776315 as i64;
    let p2_winner_universes = 341960390180808 as i64;
    let total = p1_winner_universes + p2_winner_universes;
    println!("{}", total);
}
