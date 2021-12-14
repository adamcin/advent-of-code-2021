mod common;

/// --- Day 6: Lanternfish ---
///
/// The sea floor is getting steeper. Maybe the sleigh keys got carried this way?
///
/// A massive school of glowing lanternfish swims past. They must spawn quickly to reach such 
/// large numbers - maybe exponentially quickly? You should model their growth rate to be sure.
///
/// Although you know nothing about this specific species of lanternfish, you make some guesses 
/// about their attributes. Surely, each lanternfish creates a new lanternfish once every 7 days.
///
/// However, this process isn't necessarily synchronized between every lanternfish - one lanternfish 
/// might have 2 days left until it creates another lanternfish, while another might have 4. So, 
/// you can model each fish as a single number that represents the number of days until it creates 
/// a new lanternfish.
///
/// Furthermore, you reason, a new lanternfish would surely need slightly longer before it's capable 
/// of producing more lanternfish: two more days for its first cycle.
///
/// So, suppose you have a lanternfish with an internal timer value of 3:
///
/// After one day, its internal timer would become 2.
/// After another day, its internal timer would become 1.
/// After another day, its internal timer would become 0.
/// After another day, its internal timer would reset to 6, and it would create a new lanternfish 
/// with an internal timer of 8.
/// After another day, the first lanternfish would have an internal timer of 5, and the second 
/// lanternfish would have an internal timer of 7.
/// A lanternfish that creates a new fish resets its timer to 6, not 7 (because 0 is included as 
/// a valid timer value). The new lanternfish starts with an internal timer of 8 and does not start 
/// counting down until the next day.
///
/// Realizing what you're trying to do, the submarine automatically produces a list of the ages of 
/// several hundred nearby lanternfish (your puzzle input).
fn read() -> FishPool {
    let fishes: Vec<usize> = common::read_test_input("data/day-06/input.txt")
        .iter()
        .flat_map(|line| -> Vec<usize> { line.split(',').flat_map(|s| s.parse().ok()).collect() })
        .clone()
        .collect();

    let mut cohorts: [FishCohort; 7] = [FishCohort {
        size: 0,
        maturity: 0,
    }; 7];
    for cohort_age in 0..7 {
        let aged: Vec<usize> = fishes
            .iter()
            .filter(|&age| *age == cohort_age)
            .cloned()
            .collect();
        cohorts[(cohort_age + 1) % 7] = FishCohort {
            maturity: 0,
            size: aged.len() as u128,
        };
    }
    return FishPool {
        day: 0,
        matures: cohorts,
        noobs: Vec::new(),
    };
}

/// Each day, a 0 becomes a 6 and adds a new 8 to the end of the list, while each other number 
/// decreases by 1 if it was present at the start of the day.
/// 
/// Find a way to simulate lanternfish. How many lanternfish would there be after 80 days?
#[test]
fn day06part1() {
    let mut pool = read();
    assert_eq!(300, pool.fish_count(), "expect number of fishes");
    for _ in 0..80 {
        pool.tick();
    }
    assert_eq!(365862, pool.fish_count(), "expect number of fishes");
}

/// Suppose the lanternfish live forever and have unlimited food and space. Would they take over 
/// the entire ocean?
/// 
/// How many lanternfish would there be after 256 days?
#[test]
fn day06part2() {
    let mut pool = read();
    assert_eq!(300, pool.fish_count(), "expect number of fishes");
    for _ in 0..256 {
        pool.tick();
    }

    assert_eq!(1653250886439, pool.fish_count(), "expect number of fishes");
}

struct FishPool {
    // matures[day % 7]
    day: usize,
    matures: [FishCohort; 7],
    noobs: Vec<FishCohort>,
}

impl FishPool {
    fn tick(&mut self) {
        self.day += 1;

        let mut matured: Vec<usize> = Vec::new();
        for (index, noob) in self.noobs.iter().enumerate() {
            if noob.maturity <= self.day {
                matured.push(index);
            }
        }
        let mut noob_size: u128 = 0;
        for index in matured.iter().rev() {
            let noob = self.noobs.remove(*index);
            noob_size += noob.size;
        }
        self.matures[(self.day + 2) % 7].size += noob_size;

        let baby_size = self.matures[self.day % 7].size;
        let baby = FishCohort {
            size: baby_size,
            maturity: self.day + 7,
        };
        self.noobs.push(baby);
    }

    fn fish_count(&self) -> u128 {
        return self.matures.iter().fold(0, |acc, fish| acc + fish.size)
            + self.noobs.iter().fold(0, |acc, fish| acc + fish.size);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct FishCohort {
    size: u128,
    maturity: usize,
}
