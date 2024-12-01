fn main() {
    let input = include_str!("../../inputs/day1.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i32>().unwrap());
        right.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    let sum: i32 = left
        .iter()
        .map(|l| {
            let freq = right.iter().filter(|r| *r == l).count();
            l * freq as i32
        })
        .sum();

    println!("{}", sum);
}
