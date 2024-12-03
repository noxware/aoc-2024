fn main() {
    let input = include_str!("../../inputs/day3.txt");

    let sum: i32 = input
        .split("mul(")
        .skip(1)
        .filter_map(|m| {
            let Some(closing) = m.find(")") else {
                return None;
            };

            let args = &m[0..closing];
            let Some((a, b)) = args.split_once(",") else {
                return None;
            };

            let Ok(a) = a.parse::<i32>() else {
                return None;
            };

            let Ok(b) = b.parse::<i32>() else {
                return None;
            };

            Some(a * b)
        })
        .sum();

    println!("{}", sum);
}
