fn main() {
    let input = include_str!("../../inputs/day1.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i32>().unwrap());
        right.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let distances = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs());
    let sum = distances.sum::<i32>();
    println!("{}", sum);
}
