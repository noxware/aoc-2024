use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day7.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> i64 {
    let equations = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(":").unwrap())
        .map(|(test, params)| {
            (
                test.parse::<i64>().unwrap(),
                params
                    .split_whitespace()
                    .map(|p| p.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        });

    equations
        .filter(|(test, params)| {
            std::iter::repeat(['+', '*', '|'])
                .take(params.len() - 1)
                .multi_cartesian_product()
                .any(|ops| {
                    let mut ops = ops.iter().copied();
                    let params = params.iter().copied();

                    let result = params
                        .reduce(|acc, p| {
                            let op = ops.next().unwrap();
                            eval(acc, op, p)
                        })
                        .unwrap();

                    assert_eq!(ops.next(), None);

                    result == *test
                })
        })
        .map(|(test, _)| test)
        .sum::<i64>()
}

fn eval(a: i64, op: char, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        '|' => format!("{a}{b}").parse::<i64>().unwrap(),
        _ => panic!("unknown op"),
    }
}
