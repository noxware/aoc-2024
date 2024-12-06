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

    let valid_updates = updates.filter(|pages| {
        rules.iter().all(|(before, after)| {
            let Some(before) = pages.iter().position(|p| p == before) else {
                return true;
            };

            let Some(after) = pages.iter().position(|p| p == after) else {
                return true;
            };

            before < after
        })
    });

    let middle_pages = valid_updates.map(|pages| pages[pages.len() / 2]);
    println!("{}", middle_pages.sum::<i32>());
}
