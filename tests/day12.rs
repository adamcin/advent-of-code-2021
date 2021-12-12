mod common;
use std::collections::HashMap;

/// With your submarine's subterranean subsystems subsisting suboptimally, the only way
/// you're getting out of this cave anytime soon is by finding a path yourself. Not just
/// a path - the only way to know if you've found the best path is to find all of them.
///
/// Fortunately, the sensors are still mostly working, and so you build a rough map of the
/// remaining caves (your puzzle input).
fn read() -> Vec<(String, String)> {
    let values: Vec<(String, String)> = common::read_test_input("data/day-12/input.txt")
        .iter()
        .cloned()
        .flat_map(|line| {
            let parts: Vec<&str> = line.splitn(2, "-").collect();
            if parts.len() >= 2 {
                return Some((parts[0].to_owned(), parts[1].to_owned()));
            }
            None
        })
        .collect();
    assert_eq!(25, values.len(), "expect input length");
    values
}

const START: &str = "start";
const END: &str = "end";

#[derive(Debug)]
struct Caves<'cs> {
    allow_second_small: bool,
    caves: HashMap<&'cs String, Cave<'cs>>,
}

impl<'cs> Caves<'cs> {
    fn new<'a>(pairs: &'a Vec<(String, String)>, allow_second_small: bool) -> Caves<'a> {
        let links: HashMap<&'a String, Vec<&'a String>> =
            pairs.iter().fold(HashMap::new(), |mut acc, (key, val)| {
                if acc.contains_key(key) {
                    let vals: &mut Vec<&String> = acc.get_mut(key).expect("key should be there");
                    vals.push(val);
                } else {
                    acc.insert(key, vec![val]);
                }

                if acc.contains_key(val) {
                    let keys: &mut Vec<&String> = acc.get_mut(val).expect("val should be there");
                    keys.push(key);
                } else {
                    acc.insert(val, vec![key]);
                }

                return acc;
            });
        let names: Vec<&String> = pairs.iter().flat_map(|(from, to)| vec![from, to]).collect();
        let mut caves: HashMap<&'a String, Cave<'a>> = HashMap::new();
        Caves::def_caves(&mut caves, &names, &links);
        return Caves {
            allow_second_small: allow_second_small,
            caves: caves,
        };
    }

    fn def_caves<'a>(
        caves: &mut HashMap<&'a String, Cave<'a>>,
        names: &Vec<&'a String>,
        links: &HashMap<&'a String, Vec<&'a String>>,
    ) {
        for name in names {
            let paths: Vec<&'a String> = if links.contains_key(name) {
                links.get(name).expect("expect paths for name").to_owned()
            } else {
                Vec::new()
            };
            if !caves.contains_key(name) {
                caves.insert(
                    name,
                    Cave {
                        name: name,
                        paths: paths,
                    },
                );
            }
        }
    }

    fn get_cave(&self, name: &'cs String) -> Option<&Cave<'cs>> {
        return self.caves.get(name);
    }

    fn expect_cave(&self, name: &'cs String) -> &Cave<'cs> {
        return self.get_cave(name).expect("unexpectedly missing cave");
    }

    fn find_routes(&self) -> Vec<Vec<&'cs String>> {
        let back: Vec<&'cs String> = Vec::new();
        let start_cave = self.start();
        return start_cave.follow(self, &back);
    }

    fn start(&self) -> &Cave<'cs> {
        let key = START.to_owned();
        self.caves.get(&key).expect("expected a start cave")
    }
}

#[derive(Debug)]
struct Cave<'cs> {
    name: &'cs String,
    paths: Vec<&'cs String>,
}

impl<'cs> Cave<'cs> {
    fn follow(&self, system: &Caves<'cs>, back: &Vec<&'cs String>) -> Vec<Vec<&'cs String>> {
        if Self::is_end(self.name) {
            return vec![Self::append_back(back, self.name)];
        } else if Self::check_reentry(system, back, self.name) {
            let my_back: Vec<&'cs String> = Self::append_back(back, self.name);
            return self
                .paths
                .iter()
                .map(|path| system.expect_cave(path))
                .flat_map(|cave| cave.follow(system, &my_back))
                .collect();
        } else {
            return vec![];
        }
    }

    fn contains_no_lc_dupes(back: &Vec<&'cs String>) -> bool {
        let mut acc: HashMap<&String, &String> = HashMap::new();
        for name in back.iter().filter(|name| Self::is_small(name)) {
            if acc.insert(name, name) != None {
                return false;
            }
        }
        true
    }

    fn check_reentry(system: &Caves<'cs>, back: &Vec<&'cs String>, name: &String) -> bool {
        return !Self::is_small(name)
            || !back.contains(&name)
            || system.allow_second_small && !Self::is_start(name) && Self::contains_no_lc_dupes(back);
    }

    fn append_back(back: &Vec<&'cs String>, path: &'cs String) -> Vec<&'cs String> {
        let mut route: Vec<&'cs String> = Vec::from(back.to_owned());
        route.push(path);
        return route;
    }

    fn is_start(name: &String) -> bool {
        return name == &START;
    }
    fn is_end(name: &String) -> bool {
        return name == &END;
    }

    fn is_small(name: &String) -> bool {
        return name.to_ascii_lowercase() == *name;
    }
}

/// Your goal is to find the number of distinct paths that start at start, end at end, 
/// and don't visit small caves more than once. There are two types of caves: big caves 
/// (written in uppercase, like A) and small caves (written in lowercase, like b). It 
/// would be a waste of time to visit any small cave more than once, but big caves are 
/// large enough that it might be worth visiting them multiple times. So, all paths you 
/// find should visit small caves at most once, and can visit big caves any number of times.
/// 
/// How many paths through this cave system are there that visit small caves at most once?
#[test]
fn day12part1() {
    let input = read();
    let system = Caves::new(&input, false);
    
    assert_eq!(13, system.caves.len(), "expect n caves");

    let routes = system.find_routes();
    assert_eq!(3000, routes.len(), "expect n routes");
}

/// After reviewing the available paths, you realize you might have time to visit a single 
/// small cave twice. Specifically, big caves can be visited any number of times, a single 
/// small cave can be visited at most twice, and the remaining small caves can be visited at 
/// most once. 
/// 
/// However, the caves named start and end can only be visited exactly once each: once you 
/// leave the start cave, you may not return to it, and once you reach the end cave, the path 
/// must end immediately.
/// 
/// Given these new rules, how many paths through this cave system are there?
#[test]
fn day12part2() {
    let input = read();
    let system = Caves::new(&input, true);
    
    assert_eq!(13, system.caves.len(), "expect n caves");

    let routes = system.find_routes();
    let n_routes = routes.len();
    
    assert_ne!(25508, n_routes, "too low"); // forgot to filter dupe check for lowercase names
    assert_eq!(74222, n_routes, "expect n routes");
}
