use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct Pos(i32, i32);

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
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

fn main() {
    let input = include_str!("../../inputs/day10.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> i32 {
    let mut map: HashMap<Pos, i32> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, s) in input.trim().split('\n').enumerate() {
        for (x, c) in s.chars().enumerate() {
            let pos = Pos(x as i32, y as i32);

            max_x = max_x.max(pos.x());
            max_y = max_y.max(pos.y());

            let height = c.to_digit(10).unwrap() as i32;
            map.insert(pos, height);
        }
    }

    let possible_trailheads = map
        .iter()
        .filter(|(_, height)| **height == 0)
        .map(|(pos, height)| (*pos, *height));

    possible_trailheads
        .map(|(pos, _)| {
            neighbours_with_height(&map, pos, 1)
                .map(|(pos, _)| {
                    neighbours_with_height(&map, pos, 2).map(|(pos, _)| {
                        neighbours_with_height(&map, pos, 3).map(|(pos, _)| {
                            neighbours_with_height(&map, pos, 4).map(|(pos, _)| {
                                neighbours_with_height(&map, pos, 5).map(|(pos, _)| {
                                    neighbours_with_height(&map, pos, 6).map(|(pos, _)| {
                                        neighbours_with_height(&map, pos, 7).map(|(pos, _)| {
                                            neighbours_with_height(&map, pos, 8).map(|(pos, _)| {
                                                neighbours_with_height(&map, pos, 9)
                                            })
                                        })
                                    })
                                })
                            })
                        })
                    })
                })
                .flatten()
                .flatten()
                .flatten()
                .flatten()
                .flatten()
                .flatten()
                .flatten()
                .flatten()
                .unique()
                .count() as i32
        })
        .sum::<i32>()
}

fn neighbours<'m>(map: &'m HashMap<Pos, i32>, pos: Pos) -> impl Iterator<Item = (Pos, i32)> + 'm {
    let shifts = [
        // top
        Pos(0, -1),
        // bot
        Pos(0, 1),
        // left
        Pos(-1, 0),
        //right
        Pos(1, 0),
    ];

    shifts
        .into_iter()
        .map(move |s| pos + s)
        .filter(|p| map.contains_key(&p))
        .map(|p| (p, map[&p]))
}

fn neighbours_with_height<'m>(
    map: &'m HashMap<Pos, i32>,
    pos: Pos,
    height: i32,
) -> impl Iterator<Item = (Pos, i32)> + 'm {
    neighbours(map, pos).filter(move |(_, h)| *h == height)
}
