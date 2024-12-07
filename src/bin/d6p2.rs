use std::collections::HashSet;

// These consts assume all puzzle inputs start with the guard going up.
// If that't not the case, change them.
const GUARD_CHAR: char = '^';
const INITIAL_DIRECTION: Pos = Pos(0, -1);

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
    let input = include_str!("../../inputs/day6.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> i32 {
    let mut obstructions: HashSet<Pos> = HashSet::new();
    let mut guard = Pos(0, 0);
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, s) in input.split('\n').enumerate() {
        for (x, c) in s.chars().enumerate() {
            let pos = Pos(x as i32, y as i32);

            max_x = max_x.max(pos.x());
            max_y = max_y.max(pos.y());

            match c {
                GUARD_CHAR => guard = pos,
                '#' => {
                    obstructions.insert(pos);
                }
                _ => {}
            }
        }
    }

    (0..=max_x)
        .map(|x| (0..=max_y).map(move |y| Pos(x, y)))
        .flatten()
        .filter(|pos| {
            if guard == *pos {
                return false;
            }

            if obstructions.contains(&pos) {
                return false;
            }

            obstructions.insert(*pos);
            let result = simulate(guard, &obstructions, max_x, max_y);
            obstructions.remove(pos);
            result
        })
        .count() as i32
}

fn simulate(mut guard: Pos, obstructions: &HashSet<Pos>, max_x: i32, max_y: i32) -> bool {
    let mut direction = INITIAL_DIRECTION;
    let mut visited_with_direction: HashSet<(Pos, Pos)> = HashSet::new();

    loop {
        if visited_with_direction.contains(&(guard, direction)) {
            return true;
        }

        visited_with_direction.insert((guard, direction));

        let mut looking_at = guard + direction;
        while obstructions.contains(&looking_at) {
            direction = rotate_right(direction);
            looking_at = guard + direction;
        }

        if looking_at.x() < 0
            || looking_at.y() < 0
            || looking_at.x() > max_x
            || looking_at.y() > max_y
        {
            break;
        }

        guard = looking_at;
    }

    false
}

fn rotate_right(pos: Pos) -> Pos {
    match pos {
        Pos(0, -1) => Pos(1, 0),
        Pos(1, 0) => Pos(0, 1),
        Pos(0, 1) => Pos(-1, 0),
        Pos(-1, 0) => Pos(0, -1),
        _ => panic!("not a direction"),
    }
}
