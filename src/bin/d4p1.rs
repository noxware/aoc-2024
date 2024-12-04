fn solve(input: &str) -> usize {
    let grid = input
        .split('\n')
        .enumerate()
        .map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i128, y as i128), c))
        })
        .flatten()
        .collect::<std::collections::HashMap<_, _>>();

    let x_positions = grid
        .iter()
        .filter_map(|(pos, c)| if *c == 'X' { Some(*pos) } else { None });

    let possibilities = x_positions
        .map(|(x, y)| {
            let right = [
                grid.get(&(x + 1, y)), // M
                grid.get(&(x + 2, y)), // A
                grid.get(&(x + 3, y)), // S
            ];

            let left = [
                grid.get(&(x - 1, y)), // M
                grid.get(&(x - 2, y)), // A
                grid.get(&(x - 3, y)), // S
            ];

            let down = [
                grid.get(&(x, y + 1)), // M
                grid.get(&(x, y + 2)), // A
                grid.get(&(x, y + 3)), // S
            ];

            let up = [
                grid.get(&(x, y - 1)), // M
                grid.get(&(x, y - 2)), // A
                grid.get(&(x, y - 3)), // S
            ];

            let down_right = [
                grid.get(&(x + 1, y + 1)), // M
                grid.get(&(x + 2, y + 2)), // A
                grid.get(&(x + 3, y + 3)), // S
            ];

            let down_left = [
                grid.get(&(x - 1, y + 1)), // M
                grid.get(&(x - 2, y + 2)), // A
                grid.get(&(x - 3, y + 3)), // S
            ];

            let up_right = [
                grid.get(&(x + 1, y - 1)), // M
                grid.get(&(x + 2, y - 2)), // A
                grid.get(&(x + 3, y - 3)), // S
            ];

            let up_left = [
                grid.get(&(x - 1, y - 1)), // M
                grid.get(&(x - 2, y - 2)), // A
                grid.get(&(x - 3, y - 3)), // S
            ];

            [
                right, left, down, up, down_right, down_left, up_right, up_left,
            ]
        })
        .flatten();

    let confirmed = possibilities.filter(|p| matches!(p, [Some('M'), Some('A'), Some('S')]));
    confirmed.count()
}

fn main() {
    let input = include_str!("../../inputs/day4.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r#"
.......S
......A
.....M
....XMAS
...MM
..A.A
.S..S
        "#;

        assert_eq!(solve(input), 4);
    }
}
