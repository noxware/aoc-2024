fn main() {
    let input = include_str!("../../inputs/day2.txt");

    let reports = input.split('\n').filter(|l| !l.is_empty());
    let safe = reports.filter(|l| {
        let levels = l
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut skips = 0..levels.len();
        skips.any(|s| {
            let mut levels = levels.clone();
            levels.remove(s);

            let are_levels_incresing = levels.windows(2).all(|w| w[1] > w[0]);

            levels.windows(2).all(|w| {
                let is_window_incresing = w[1] > w[0];
                let distance = (w[1] - w[0]).abs();

                distance >= 1 && distance <= 3 && is_window_incresing == are_levels_incresing
            })
        })
    });

    println!("{}", safe.count())
}
