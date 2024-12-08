use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day8.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct Pos(i32, i32);

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Pos {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }
}

fn solve(input: &str) -> i32 {
    let mut antennas: Vec<(Pos, char)> = Vec::new();
    let mut frequencies: HashSet<char> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, s) in input.trim().split('\n').enumerate() {
        for (x, c) in s.chars().enumerate() {
            let pos = Pos(x as i32, y as i32);

            max_x = max_x.max(pos.x());
            max_y = max_y.max(pos.y());

            match c {
                '.' => {}
                a => {
                    frequencies.insert(a);
                    antennas.push((pos, a));
                }
            }
        }
    }

    frequencies
        .iter()
        .map(|f| {
            antennas
                .iter()
                .filter(|a| a.1 == *f)
                .map(|a| a.0)
                .permutations(2)
                .map(|pair| {
                    antinodes(pair[0], pair[1]).into_iter().filter(|antinode| {
                        antinode.x() >= 0
                            && antinode.x() <= max_x
                            && antinode.y() >= 0
                            && antinode.y() <= max_y
                    })
                })
                .flatten()
        })
        .flatten()
        .unique()
        .count() as i32
}

fn antinodes(a: Pos, b: Pos) -> [Pos; 2] {
    let a_antinode = a + (a - b);
    let b_antinode = b + (b - a);
    [a_antinode, b_antinode]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antinodes() {
        let res = antinodes(Pos(3, 2), Pos(4, 1));
        assert_eq!(res, [Pos(2, 3), Pos(5, 0)]);
    }
}
