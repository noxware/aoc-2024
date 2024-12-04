fn main() {
    let input = include_str!("../../inputs/day4.txt");

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

    let a_positions = grid
        .iter()
        .filter_map(|(pos, c)| if *c == 'A' { Some(*pos) } else { None });

    let confirmed = a_positions.filter(|(x, y)| {
        // (-1, -1)         (1, -1)
        //          A(0, 0)
        // (-1, 1)          (1, 1)
        let possibilities = [
            [
                (-1, -1), // M
                (1, 1),   // S
            ],
            [
                (1, 1),   // M
                (-1, -1), // S
            ],
            [
                (-1, 1), // M
                (1, -1), // S
            ],
            [
                (1, -1), // M
                (-1, 1), // S
            ],
        ]
        .map(|p| p.map(|(px, py)| (x + px, y + py)));

        let mas_count = possibilities
            .iter()
            .filter(|p| matches!([grid.get(&p[0]), grid.get(&p[1])], [Some('M'), Some('S')]))
            .count();

        mas_count == 2
    });

    println!("{}", confirmed.count());
}
