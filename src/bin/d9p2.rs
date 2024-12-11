use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day9.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> i64 {
    let mut ids = 0..;

    let mut disk: Vec<(Option<i64>, i64)> = input
        .chars()
        .filter_map(|size| size.to_digit(10).map(|d| d as i64))
        .enumerate()
        .map(|(idx, size)| {
            if idx % 2 == 0 {
                let id = ids.next().unwrap();
                (Some(id as i64), size)
            } else {
                (None, size)
            }
        })
        .collect();

    let mut tried_files: HashSet<i64> = HashSet::new();

    loop {
        // let's find a file that we didn't tried to move yet.
        let Some(file_idx) = disk.iter().rposition(|(id, _)| {
            let Some(id) = id else {
                return false;
            };

            !tried_files.contains(id)
        }) else {
            // if we already tried moving all files, we finished here
            break;
        };

        // let's take out the file data, leaving free space where it was
        let file_id = disk[file_idx].0.unwrap();
        let file_size = disk[file_idx].1;
        disk[file_idx] = (None, file_size);

        // let's mark the file as checked, before we forget
        tried_files.insert(file_id);

        // let's find the first free space from the right that can hold the whole file.
        // at least, it should match the blank space we left before making no changes.
        let free_idx = disk
            .iter()
            .position(|f| f.0.is_none() && f.1 >= file_size)
            .unwrap();

        // let's shrink the free space
        disk[free_idx].1 -= file_size;

        // let's insert the file before the shrinked free space
        disk.insert(free_idx, (Some(file_id), file_size));
    }

    let unpacked = disk
        .iter()
        .map(|(id, size)| {
            if let Some(id) = id {
                std::iter::repeat(Some(*id)).take(*size as usize)
            } else {
                std::iter::repeat(None).take(*size as usize)
            }
        })
        .flatten();

    unpacked
        .enumerate()
        .map(|(idx, b)| idx as i64 * b.unwrap_or(0))
        .sum::<i64>()
}
