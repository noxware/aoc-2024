fn main() {
    let input = include_str!("../../inputs/day9.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> i64 {
    let mut ids = 0..;

    let disk: Vec<Option<i64>> = input
        .chars()
        .filter_map(|size| size.to_digit(10))
        .enumerate()
        .map(|(idx, size)| {
            if idx % 2 == 0 {
                let id = ids.next().unwrap();
                std::iter::repeat(Some(id)).take(size as usize)
            } else {
                std::iter::repeat(None).take(size as usize)
            }
        })
        .flatten()
        .collect();

    let mut rightmost_files = disk.iter().rev().filter_map(|b| *b);
    let used = disk.iter().filter_map(|b| *b).count();

    (0..used)
        .map(|idx| disk[idx].or_else(|| rightmost_files.next()).unwrap())
        .enumerate()
        .map(|(idx, b)| idx as i64 * b)
        .sum::<i64>()
}
