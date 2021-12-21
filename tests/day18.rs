mod common;

// --- Day 18: Snailfish ---

/// You descend into the ocean trench and encounter some snailfish. They say they saw the sleigh
/// keys! They'll even tell you which direction the keys went if you help one of the smaller
/// snailfish with his math homework.

/// Snailfish numbers aren't like regular numbers. Instead, every snailfish number is a pair -
/// an ordered list of two elements. Each element of the pair can be either a regular number or
/// another pair.

/// Pairs are written as [x,y], where x and y are the elements within the pair. Here are some
/// example snailfish numbers, one snailfish number per line:

/// [1,2]
/// [[1,2],3]
/// [9,[8,7]]
/// [[1,9],[8,5]]
/// [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
/// [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
/// [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
///
/// This snailfish homework is about addition. To add two snailfish numbers, form a pair from
/// the left and right parameters of the addition operator. For example, [1,2] + [[3,4],5]
/// becomes [[1,2],[[3,4],5]].

/// There's only one problem: snailfish numbers must always be reduced, and the process of adding
/// two snailfish numbers can result in snailfish numbers that need to be reduced.

/// To reduce a snailfish number, you must repeatedly do the first action in this list that
/// applies to the snailfish number:

// If any pair is nested inside four pairs, the leftmost such pair explodes.
// If any regular number is 10 or greater, the leftmost such regular number splits.
// Once no action in the above list applies, the snailfish number is reduced.

/// During reduction, at most one action applies, after which the process returns to the top of
/// the list of actions. For example, if split produces a pair that meets the explode criteria,
/// that pair explodes before other splits occur.

/// To explode a pair, the pair's left value is added to the first regular number to the left of
/// the exploding pair (if any), and the pair's right value is added to the first regular number
/// to the right of the exploding pair (if any). Exploding pairs will always consist of two regular
/// numbers. Then, the entire exploding pair is replaced with the regular number 0.

// Here are some examples of a single explode action:

// [[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4] (the 9 has no regular number to its left, so it is not added to any regular number).
// [7,[6,[5,[4,[3,2]]]]] becomes [7,[6,[5,[7,0]]]] (the 2 has no regular number to its right, and so it is not added to any regular number).
// [[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3].
// [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] (the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action).
// [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[7,0]]]].
// To split a regular number, replace it with a pair; the left element of the pair should be the regular number divided by two and rounded down, while the right element of the pair should be the regular number divided by two and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.

// Here is the process of finding the reduced result of [[[[4,3],4],4],[7,[[8,4],9]]] + [1,1]:

// after addition: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
// after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
// after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
// after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
// after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
// after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
// Once no reduce actions apply, the snailfish number that remains is the actual result of the addition operation: [[[[0,7],4],[[7,8],[6,0]]],[8,1]].

/// The homework assignment involves adding up a list of snailfish numbers (your puzzle input).
/// The snailfish numbers are each listed on a separate line. Add the first snailfish number and
/// the second, then add that result and the third, then add that result and the fourth, and so
/// on until all numbers in the list have been used once.
///
/// note: this should be represented as a Binary Tree based on the instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Nature {
    Pair,
    Leaf(usize),
}
use Nature::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    nature: Nature,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}
use Dir::*;

impl Node {
    fn len(&self) -> usize {
        match self.nature {
            Pair => {
                self.left.as_ref().map(|n| n.len()).unwrap_or(0)
                    + self.right.as_ref().map(|n| n.len()).unwrap_or(0)
            }
            Leaf(_) => 1,
        }
    }

    fn copy(&self) -> Node {
        match self.nature {
            Leaf(value) => Node::new_leaf(value),
            Pair => Node::new(
                self.left.as_ref().map(|n| n.copy()),
                self.right.as_ref().map(|n| n.copy()),
            ),
        }
    }

    fn new_leaf(value: usize) -> Node {
        return Node {
            left: None,
            right: None,
            nature: Leaf(value),
        };
    }

    fn new_leaves(lvalue: usize, rvalue: usize) -> Node {
        Self::new(Some(Self::new_leaf(lvalue)), Some(Self::new_leaf(rvalue)))
    }

    fn new(left: Option<Node>, right: Option<Node>) -> Node {
        return Node {
            left: left.map(|n| Box::new(n)),
            right: right.map(|n| Box::new(n)),
            nature: Pair,
        };
    }

    fn new_empty() -> Node {
        Self::new(None, None)
    }

    fn can_split(&self) -> bool {
        match self.nature {
            Leaf(value) => value >= 10,
            _ => false,
        }
    }

    /// To split a regular number, replace it with a pair; the left element of the pair
    /// should be the regular number divided by two and rounded down, while the right
    /// element of the pair should be the regular number divided by two and rounded up.
    /// For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.
    fn split(&mut self) -> bool {
        match self.nature {
            Leaf(value) => {
                value >= 10 && {
                    let lval = value / 2;
                    let rval = (value / 2) + (value % 2);
                    self.left = Some(Box::new(Node::new_leaf(lval)));
                    self.right = Some(Box::new(Node::new_leaf(rval)));
                    self.nature = Pair;
                    true
                }
            }
            _ => false,
        }
    }

    fn is_leaf(&self) -> bool {
        match self.nature {
            Leaf(_) => true,
            _ => false,
        }
    }

    fn is_pair_of_leaves(&self) -> bool {
        self.left.as_ref().map(|n| n.is_leaf()).unwrap_or(false)
            && self.right.as_ref().map(|n| n.is_leaf()).unwrap_or(false)
    }

    /// no nead to leak a reference to a read-only pair
    fn _pair_at(&self, index: usize, depth: usize) -> Option<(usize, Dir)> {
        assert!(
            index < self.len(),
            "Node pair_at index {} exceeds len {}",
            index,
            self.len()
        );
        match self.nature {
            Leaf(_) => None,
            Pair => {
                if self.is_pair_of_leaves() {
                    let dir = if index == 0 { Left } else { Right };
                    Some((depth, dir))
                } else {
                    let left_len = self.left.as_ref().map(|n| n.len()).unwrap_or(0);
                    if index < left_len {
                        self.left.as_ref().unwrap()._pair_at(index, depth + 1)
                    } else {
                        self.right
                            .as_ref()
                            .unwrap()
                            ._pair_at(index - left_len, depth + 1)
                    }
                }
            }
        }
    }

    fn _pair_at_mut(&mut self, index: usize, depth: usize) -> Option<(&mut Node, usize)> {
        assert!(
            index < self.len(),
            "Node pair_at_mut index {} exceeds len {}",
            index,
            self.len()
        );
        match self.nature {
            Leaf(_) => None,
            Pair => {
                if self.is_pair_of_leaves() {
                    Some((self, depth))
                } else {
                    let left_len = self.left.as_ref().map(|n| n.len()).unwrap_or(0);
                    if index < left_len {
                        self.left.as_mut().unwrap()._pair_at_mut(index, depth + 1)
                    } else {
                        self.right
                            .as_mut()
                            .unwrap()
                            ._pair_at_mut(index - left_len, depth + 1)
                    }
                }
            }
        }
    }

    fn leaf_value(&self) -> Option<usize> {
        match self.nature {
            Leaf(value) => Some(value),
            _ => None,
        }
    }

    /// internal explode.
    /// if self is pair of leaves, change nature to Leaf(0), and return previous leaf
    /// values as tuple. should only be called by Tree.explode().
    fn _explode(&mut self) -> Option<(usize, usize)> {
        if self.is_pair_of_leaves() {
            self.nature = Leaf(0);
            let leaf_values = self
                .left
                .as_ref()
                .and_then(|n| n.leaf_value())
                .zip(self.right.as_ref().and_then(|n| n.leaf_value()));
            self.left = None;
            self.right = None;
            return leaf_values;
        } else {
            None
        }
    }

    /// To check whether it's the right answer, the snailfish teacher only checks the magnitude
    /// of the final sum. The magnitude of a pair is 3 times the magnitude of its left element
    /// plus 2 times the magnitude of its right element. The magnitude of a regular number is
    /// just that number.
    fn magnitude(&self) -> usize {
        match self.nature {
            Leaf(value) => value,
            Pair => {
                let lval = self.left.as_ref().map(|n| n.magnitude()).unwrap_or(0);
                let rval = self.right.as_ref().map(|n| n.magnitude()).unwrap_or(0);
                (3 * lval) + (2 * rval)
            }
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self.nature {
            Leaf(value) => value.to_string(),
            Pair => format!(
                "[{},{}]",
                self.left
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or("?".to_owned()),
                self.right
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or("?".to_owned())
            ),
        }
    }
}

///
/// For example, the magnitude of [9,1] is 3*9 + 2*1 = 29;
/// the magnitude of [1,9] is 3*1 + 2*9 = 21.
///
/// Magnitude calculations are recursive:
/// the magnitude of [[9,1],[1,9]] is 3*29 + 2*21 = 129.
///
/// Here are a few more magnitude examples:
///
/// [[1,2],[[3,4],5]] becomes 143.
/// [[[[0,7],4],[[7,8],[6,0]]],[8,1]] becomes 1384.
/// [[[[1,1],[2,2]],[3,3]],[4,4]] becomes 445.
/// [[[[3,0],[5,3]],[4,4]],[5,5]] becomes 791.
/// [[[[5,0],[7,4]],[5,5]],[6,6]] becomes 1137.
/// [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] becomes 3488.
#[test]
fn day18_unit_node_magnitude() {
    assert_eq!(
        29,
        Node::new_leaves(9, 1).magnitude(),
        "expect mag for [9,1]"
    );
    assert_eq!(
        21,
        Node::new_leaves(1, 9).magnitude(),
        "expect mag for [1,9]"
    );
    assert_eq!(
        129,
        Node::new(Some(Node::new_leaves(9, 1)), Some(Node::new_leaves(1, 9))).magnitude(),
        "expect mag for [[9,1],[1,9]]"
    );
}

/// To split a regular number, replace it with a pair; the left element of the pair
/// should be the regular number divided by two and rounded down, while the right
/// element of the pair should be the regular number divided by two and rounded up.
/// For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.
#[test]
fn day18_unit_node_split() {
    let mut ten = Node::new_leaf(10);
    assert!(ten.split(), "expect ten split");
    assert_ne!(
        Node::new_leaf(10),
        ten,
        "ten should not equal its previous state"
    );
    assert_eq!(Node::new_leaves(5, 5), ten, "ten should equal split");

    let mut nine = Node::new_leaf(9);
    assert!(!nine.split(), "expect 9 not split");
    assert_eq!(Node::new_leaf(9), nine, "expect nine equal previous state");

    let mut eleven = Node::new_leaf(11);
    assert!(eleven.split(), "expect eleven split");
    assert_ne!(
        Node::new_leaf(11),
        eleven,
        "eleven should not equal previous state"
    );
    assert_eq!(Node::new_leaves(5, 6), eleven, "eleven should equal split");

    let mut twelve = Node::new_leaf(12);
    assert!(twelve.split(), "expect twelve split");
    assert_ne!(
        Node::new_leaf(12),
        twelve,
        "twelve should not equal previous state"
    );
    assert_eq!(Node::new_leaves(6, 6), twelve, "twelve should equal split");
}

use std::ops::Index;
use std::ops::IndexMut;

impl Index<usize> for Node {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < self.len(),
            "Node index {} exceeds len {}",
            index,
            self.len()
        );
        match self.nature {
            Leaf(_) => self,
            Pair => {
                let left_len = self.left.as_ref().map(|n| n.len()).unwrap_or(0);
                if index < left_len {
                    self.left.as_ref().unwrap().index(index)
                } else {
                    self.right.as_ref().unwrap().index(index - left_len)
                }
            }
        }
    }
}

impl IndexMut<usize> for Node {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            index < self.len(),
            "Node index {} exceeds len {}",
            index,
            self.len()
        );
        match self.nature {
            Leaf(_) => self,
            Pair => {
                let left_len = self.left.as_ref().map(|n| n.len()).unwrap_or(0);
                if index < left_len {
                    self.left.as_mut().unwrap().index_mut(index)
                } else {
                    self.right.as_mut().unwrap().index_mut(index - left_len)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tree {
    root: Node,
}

impl Tree {
    fn len(&self) -> usize {
        return self.root.len();
    }

    fn copy(&self) -> Tree {
        return Tree {
            root: self.root.copy(),
        };
    }

    fn add_reduce(&self, right: &Self, print: bool) -> Self {
        println!("  {}", self.to_string());
        println!("+ {}", right.to_string());
        let mut new_tree = self.add(right);
        while new_tree.reduce(print) {}
        println!("= {}", new_tree.to_string());
        println!("");
        return new_tree;
    }

    fn add(&self, right: &Self) -> Self {
        Self {
            root: Node::new(Some(self.root.copy()), Some(right.root.copy())),
        }
    }

    fn update(&mut self, index: Option<usize>, value: usize) {
        if let Some(index) = index {
            if let Leaf(orig) = self.index(index).nature {
                self.index_mut(index).nature = Leaf(orig + value);
            }
        }
    }

    fn magnitude(&self) -> usize {
        return self.root.magnitude();
    }

    fn reduce(&mut self, print: bool) -> bool {
        if print {
            println!(". {}", self.to_string());
        }
        self.explode() || self.split()
    }

    fn split(&mut self) -> bool {
        for i in 0..self.len() {
            if self.index(i).can_split() {
                return self.index_mut(i).split();
            }
        }
        false
    }

    fn explode(&mut self) -> bool {
        let orig = self.len();
        for i in 0..orig {
            if let Some((depth, dir)) = self.root._pair_at(i, 0) {
                if depth >= 4 {
                    // i represents index of either a left or right leaf
                    // on the left side, the indices will be stable:
                    // if dir == Left, i - 1
                    // if dir == Right, i - 2
                    // on the right side, the new indicies will shift down by one
                    // if dir == Left, i + 1
                    // if dir == Right, i
                    let next_left_index: Option<usize> = (match dir {
                        Left => Some((i as i128) - 1),
                        Right => Some((i as i128) - 2),
                    })
                    .filter(|v| *v >= 0)
                    .map(|v| v as usize);
                    let next_right_index: Option<usize> = (match dir {
                        Left => Some((i as i128) + 1),
                        Right => Some(i as i128),
                    })
                    .filter(|v| *v < (orig as i128) - 1)
                    .map(|v| v as usize);
                    if let Some((pair, _)) = self.root._pair_at_mut(i, 0) {
                        if let Some((lvalue, rvalue)) = pair._explode() {
                            self.update(next_left_index, lvalue);
                            self.update(next_right_index, rvalue);
                            return true;
                        }
                    }
                    return false;
                }
            }
        }
        false
    }

    fn read(expr: &str) -> Option<Tree> {
        let mut stack: Vec<Node> = Vec::new();
        let mut stack_dir: Vec<Dir> = Vec::new();
        for ch in expr.chars() {
            if ch.is_digit(10) {
                if let (Some(mut parent), Some(dir)) = (stack.last_mut(), stack_dir.last()) {
                    if let Some(value) = String::from_iter(vec![ch].iter()).parse().ok() {
                        match dir {
                            Left => parent.left = Some(Box::new(Node::new_leaf(value))),
                            Right => parent.right = Some(Box::new(Node::new_leaf(value))),
                        }
                    }
                }
            } else if ch == ',' {
                stack_dir.pop();
                stack_dir.push(Right);
            } else if ch == '[' {
                stack.push(Node::new_empty());
                stack_dir.push(Left);
            } else if ch == ']' {
                if let (Some(top), Some(_)) = (stack.pop(), stack_dir.pop()) {
                    if let (Some(parent), Some(dir)) = (stack.last_mut(), stack_dir.last()) {
                        match dir {
                            Left => parent.left = Some(Box::new(top)),
                            Right => parent.right = Some(Box::new(top)),
                        };
                    } else {
                        return Some(Tree { root: top });
                    }
                }
            }
        }
        None
    }
}

impl ToString for Tree {
    fn to_string(&self) -> String {
        return self.root.to_string();
    }
}

#[test]
fn day18_unit_tree_read() {
    //let tree = Tree::read("[0,1]").unwrap();
    //assert_eq!(Node::new_leaves(0, 1), tree.root, "expect correct reading");

    let tree2 = Tree::read("[[2,[2,[4,0]]],[6,1]]").unwrap();
    assert_eq!(
        Node::new(
            Some(Node::new(
                Some(Node::new_leaf(2)),
                Some(Node::new(
                    Some(Node::new_leaf(2)),
                    Some(Node::new_leaves(4, 0))
                ))
            )),
            Some(Node::new_leaves(6, 1))
        ),
        tree2.root,
        "expect correct reading",
    );
    println!("tree2: {}", tree2.to_string());
}

///  [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
// + [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
// = [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]

//   [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
// + [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
// = [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]

//   [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
// + [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
// = [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]

//   [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
// + [7,[5,[[3,8],[1,4]]]]
// = [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]

//   [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
// + [[2,[2,2]],[8,[8,1]]]
// = [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]

//   [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
// + [2,9]
// = [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]

//   [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
// + [1,[[[9,3],9],[[9,0],[0,7]]]]
// = [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]

//   [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
// + [[[5,[7,4]],7],1]
// = [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]

//   [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
// + [[[[4,2],2],6],[8,7]]
// = [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
#[test]
fn day18_unit_tree_final_sum() {
    let input_string = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    [7,[5,[[3,8],[1,4]]]]
    [[2,[2,2]],[8,[8,1]]]
    [2,9]
    [1,[[[9,3],9],[[9,0],[0,7]]]]
    [[[5,[7,4]],7],1]
    [[[[4,2],2],6],[8,7]]";

    let input: Vec<Tree> = input_string
        .split_whitespace()
        .flat_map(|line| Tree::read(line))
        .collect();
    if let Some((head, tail)) = input.split_first() {
        let result = tail
            .iter()
            .fold(head.copy(), |acc, v| acc.add_reduce(v, true));
        println!(
            "result magnitude {} <= {}",
            result.magnitude(),
            result.to_string()
        );

        assert_eq!(3488, result.magnitude(), "expect magnitude");
    }
}

impl Index<usize> for Tree {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < self.len(),
            "Tree index {} exceeds len {}",
            index,
            self.len()
        );
        return self.root.index(index);
    }
}

impl IndexMut<usize> for Tree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            index < self.len(),
            "Tree index {} exceeds len {}",
            index,
            self.len()
        );
        return self.root.index_mut(index);
    }
}

fn read() -> Vec<Tree> {
    common::read_test_input("data/day-18/input.txt")
        .iter()
        .flat_map(|line| Tree::read(&line[0..]))
        .collect()
}

#[test]
fn day18part1() {
    let input = read();
    assert_eq!(100, input.len(), "expect lines of input");

    if let Some((head, tail)) = input.split_first() {
        let result = tail
            .iter()
            .fold(head.copy(), |acc, v| acc.add_reduce(v, false));
        println!(
            "result magnitude {} <= {}",
            result.magnitude(),
            result.to_string()
        );

        assert_ne!(3450, result.magnitude(), "too low");
    }
}

/// What is the largest magnitude of any sum of two different snailfish numbers from the homework assignment?
#[test]
fn day18part2() {
    let mut mags: Vec<usize> = Vec::new();
    let input = read();
    let compr: Vec<Tree> = input.iter().map(|t| t.copy()).collect();
    for left in &input {
        for right in &compr {
            if left != right {
                mags.push(std::cmp::max(
                    left.add_reduce(&right, false).magnitude(),
                    right.add_reduce(&left, false).magnitude(),
                ));
            }
        }
    }
    let maximum = mags.iter().fold(0, |a, v| std::cmp::max(a, *v));
    assert_eq!(4483, maximum, "expect maximum magnitude");
}
