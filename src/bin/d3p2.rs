fn normalize(input: &str) -> String {
    let mut pending = input;
    let mut ignoring = false;
    let mut normalized = String::new();

    loop {
        if ignoring {
            let Some(dox) = pending.find("do()") else {
                break;
            };

            pending = &pending[dox..];
            ignoring = false;
        } else {
            let Some(dont) = pending.find("don't()") else {
                normalized.push_str(pending);
                break;
            };

            normalized.push_str(&pending[..dont]);
            pending = &pending[dont..];
            ignoring = true;
        }
    }

    normalized
}

fn solve(input: &str) -> i32 {
    normalize(input)
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
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day3.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "don't()mul(1,1)do()mul(1,1)do()don't()mul(1,1)";
        assert_eq!(solve(input), 1);
    }
}
