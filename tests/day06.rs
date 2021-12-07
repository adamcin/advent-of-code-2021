#[cfg(test)]
mod day06test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    struct LanternfishPool {
        fishes: Vec<Lanternfish>,
    }

    impl LanternfishPool {
        fn fish_count(&self) -> usize {
            return self.fishes.len();
        }

        fn tick(&mut self) {
            let mut new_fishes: Vec<Lanternfish> = Vec::new();
            for fish in self.fishes.iter_mut() {
                if fish.tick() == 0 {
                    new_fishes.push(Lanternfish { age: 8 });
                }
            }
            for fish in new_fishes {
                self.fishes.push(fish);
            }
        }
    }

    struct Lanternfish {
        age: usize,
    }

    impl Lanternfish {
        fn tick(&mut self) -> usize {
            let old_age = self.age;
            if self.age > 0 {
                self.age -= 1;
            } else {
                self.age = 6;
            }
            return old_age;
        }
    }

    fn day06read_legacy(filename: &str) -> LanternfishPool {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().flat_map(|line_r| line_r.ok()).collect();

        let fishes: Vec<Lanternfish> = lines
            .iter()
            .flat_map(|line| -> Vec<usize> {
                line.split(',').flat_map(|s| s.parse().ok()).collect()
            })
            .clone()
            .map(|age| Lanternfish { age: age })
            .collect();

        return LanternfishPool { fishes: fishes };
    }

    fn day06read(filename: &str) -> FishPool {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().flat_map(|line_r| line_r.ok()).collect();

        let fishes: Vec<usize> = lines
            .iter()
            .flat_map(|line| -> Vec<usize> {
                line.split(',').flat_map(|s| s.parse().ok()).collect()
            })
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

    #[test]
    fn day06part1() {
        let mut pool = day06read("data/day-06/input.txt");
        assert_eq!(300, pool.fish_count(), "expect number of fishes");
        for day in 0..80 {
            pool.tick();
            if day == 0 {
                assert_eq!(
                    300,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 1 {
                assert_eq!(
                    430,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 2 {
                assert_eq!(
                    474,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 3 {
                assert_eq!(
                    510,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 4 {
                assert_eq!(
                    561,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 5 {
                assert_eq!(
                    600,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 6 {
                assert_eq!(
                    600,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 7 {
                assert_eq!(
                    600,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 8 {
                assert_eq!(
                    730,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 14 {
                assert_eq!(
                    1200,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 20 {
                assert_eq!(
                    2196,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 40 {
                assert_eq!(
                    12211,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            } else if day == 79 {
                assert_eq!(
                    365862,
                    pool.fish_count(),
                    "expect number of fishes after day {}",
                    day + 1
                );
            }
        }

        assert_ne!(388430, pool.fish_count(), "too high");
        assert_eq!(365862, pool.fish_count(), "expect number of fishes");
    }

    struct FishPool {
        // matures[day % 6]
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

    #[test]
    fn day06part2() {
        let mut pool = day06read("data/day-06/input.txt");
        assert_eq!(300, pool.fish_count(), "expect number of fishes");
        for _ in 0..256 {
            pool.tick();
        }

        assert_eq!(1653250886439, pool.fish_count(), "expect number of fishes");
    }
}
