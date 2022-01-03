mod common;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Player {
    id: usize,
    turns: usize, // n of turns completed before this turn
    index: usize,
    score: usize,
}

impl Player {
    fn new(id: usize, pos: usize, turns: usize) -> Self {
        assert!(pos >= 1 && pos <= 10);
        Self {
            id: id,
            turns: turns,
            index: pos - 1,
            score: 0,
        }
    }

    fn play(&self, spaces: usize) -> Self {
        let new_index = (self.index + spaces) % 10;
        Self {
            id: self.id,
            turns: self.turns + 1,
            index: new_index,
            score: self.score + new_index + 1,
        }
    }

    fn get_score(&self) -> usize {
        self.score
    }

    fn num_rolls(&self) -> usize {
        self.turns * 3
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Die {
    Determ { seed: usize, side: usize },
    Hyper { comb: u128 },
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
            Hyper { comb: prev_comb } => vec![
                // 3 (1,1,1): 1
                (
                    Hyper {
                        comb: prev_comb * 1,
                    },
                    player.play(3),
                ),
                // 4 (2,1,1) (1,2,1) (1,1,2): 3
                (
                    Hyper {
                        comb: prev_comb * 3,
                    },
                    player.play(4),
                ),
                // 5 (2,2,1) (2,1,2) (1,2,2) (3,1,1) (1,3,1) (1,1,3): 6
                (
                    Hyper {
                        comb: prev_comb * 6,
                    },
                    player.play(5),
                ),
                // 6 (2,2,2) (1,2,3) (1,3,2) (2,1,3) (2,3,1) (3,1,2) (3,2,1): 7
                (
                    Hyper {
                        comb: prev_comb * 7,
                    },
                    player.play(6),
                ),
                // 7 (1,3,3) (3,1,3) (3,3,1) (3,2,2) (2,3,2) (2,2,3): 6
                (
                    Hyper {
                        comb: prev_comb * 6,
                    },
                    player.play(7),
                ),
                // 8 (2,3,3) (3,2,3) (3,3,2): 3
                (
                    Hyper {
                        comb: prev_comb * 3,
                    },
                    player.play(8),
                ),
                // 9 (3,3,3): 1
                (
                    Hyper {
                        comb: prev_comb * 1,
                    },
                    player.play(9),
                ),
            ],
        }
    }

    fn get_comb(&self) -> u128 {
        match self {
            Hyper { comb } => *comb,
            _ => 0,
        }
    }
}

fn read() -> (Player, Player) {
    (Player::new(1, 9, 0), Player::new(2, 3, 0))
}

fn read_test() -> (Player, Player) {
    (Player::new(1, 4, 0), Player::new(2, 8, 0))
}

fn determ_play_until_score(
    win_score: usize,
    player1: &Player,
    player2: &Player,
) -> (Player, Player) {
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

fn hyper_play_until_score(
    win_score: usize,
    die: &Die,
    player_up: &Player,
    player_last: &Player,
) -> (u128, u128) {
    if player_last.get_score() >= win_score {
        if player_last.id == 1 {
            return (die.get_comb(), 0);
        } else {
            return (0, die.get_comb());
        }
    }

    die.roll(player_up)
        .iter()
        .map(|(next_die, player_played)| {
            hyper_play_until_score(win_score, next_die, player_last, player_played)
        })
        .fold((0 as u128, 0 as u128), |(p1a, p2a), (p1v, p2v)| {
            (p1a + p1v, p2a + p2v)
        })
}

#[test]
fn day21pre_part1() {
    let (og_player1, og_player2) = read_test();
    let (player1, player2) = determ_play_until_score(1000, &og_player1, &og_player2);

    assert_eq!(
        739785,
        std::cmp::min(player1.score, player2.score) * (player1.num_rolls() + player2.num_rolls())
    );
}

#[test]
fn day21part1() {
    let (og_player1, og_player2) = read();
    let (player1, player2) = determ_play_until_score(1000, &og_player1, &og_player2);

    assert_eq!(
        1073709,
        std::cmp::min(player1.score, player2.score) * (player1.num_rolls() + player2.num_rolls())
    );
}

#[test]
fn day21pre_part2() {
    let expect_p1_wins = 444356092776315 as u128;
    let expect_p2_wins = 341960390180808 as u128;
    let (player1, player2) = read_test();

    let (p1_wins, p2_wins) = hyper_play_until_score(21, &Hyper { comb: 1 }, &player1, &player2);
    println!("complete.");
    assert_eq!((expect_p1_wins, expect_p2_wins), (p1_wins, p2_wins));
}


#[test]
fn day21part2() {
    let expect_p1_wins = 148747830493442 as u128;
    let expect_p2_wins = 89305072914203 as u128;
    let (player1, player2) = read();

    let (p1_wins, p2_wins) = hyper_play_until_score(21, &Hyper { comb: 1 }, &player1, &player2);
    println!("complete.");
    assert_eq!((expect_p1_wins, expect_p2_wins), (p1_wins, p2_wins));
}


