use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let (rules, updates) = input.split_once("\n\n").expect("invalid input format");

    let rules = rules
        .split("\n")
        .filter(|r| !r.is_empty())
        .map(|r| r.split_once('|').expect("invalid format"))
        .map(|(before, after)| {
            (
                before.parse::<i32>().expect("not a before number"),
                after.parse::<i32>().expect("not an after number"),
            )
        })
        .collect::<Vec<_>>();

    let updates = updates.split("\n").filter(|u| !u.is_empty()).map(|u| {
        u.split(',')
            .map(|page| page.parse::<i32>().expect("not a page number"))
            .collect::<Vec<_>>()
    });

    let invalid_updates = updates.filter(|pages| {
        !rules.iter().all(|(before, after)| {
            let Some(before) = pages.iter().position(|p| p == before) else {
                return true;
            };

            let Some(after) = pages.iter().position(|p| p == after) else {
                return true;
            };

            before < after
        })
    });

    let fixed_updates = invalid_updates.map(|mut pages| {
        pages.sort_by(|a, b| {
            let rule = rules
                .iter()
                .find(|(before, after)| (a == before || a == after) && (b == before || b == after));

            let Some((before, _after)) = rule else {
                return Ordering::Equal;
            };

            if before == a {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });

        pages
    });

    let middle_pages = fixed_updates.map(|pages| pages[pages.len() / 2]);
    println!("{}", middle_pages.sum::<i32>());
}
