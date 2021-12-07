#[cfg(test)]
mod day06test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
        for _ in 0..80 {
            pool.tick();
        }
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
