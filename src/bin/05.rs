use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.split("\n\n");

    let raw_ranges = input.next().unwrap();
    let raw_vals = input.next().unwrap();

    let mut ranges = Vec::with_capacity(raw_ranges.len());

    raw_ranges
        .split("\n")
        .filter(|test| !test.is_empty())
        .for_each(|e| {
            let mut a = e.split("-");

            let min = a.next().unwrap().parse::<u64>().unwrap();
            let max = a.next().unwrap().parse::<u64>().unwrap();

            ranges.push(min..=max);
        });

    let res = raw_vals
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| e.parse::<u64>().unwrap())
        .fold(0_u64, |prev, product| {
            if ranges.iter().any(|e| e.contains(&product)) {
                prev + 1
            } else {
                prev
            }
        });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.split("\n\n");

    let raw_ranges = input.next().unwrap();

    let mut ranges = Vec::with_capacity(raw_ranges.len());

    raw_ranges
        .split("\n")
        .filter(|test| !test.is_empty())
        .for_each(|e| {
            let mut a = e.split("-");

            let min = a.next().unwrap().parse::<u64>().unwrap();
            let max = a.next().unwrap().parse::<u64>().unwrap();

            ranges.push(min..=max);
        });

    loop {
        let mut merge: Option<(RangeInclusive<u64>, RangeInclusive<u64>)> = None;

        'outer: for (skip_amount, range_a) in ranges.iter().enumerate() {
            for range_b in ranges.iter().skip(skip_amount + 1) {
                let overlap = range_b.contains(range_a.start())
                    || range_b.contains(range_a.end())
                    || range_a.contains(range_b.start())
                    || range_a.contains(range_b.end());

                if overlap {
                    merge = Some((range_a.clone(), range_b.clone()));
                    break 'outer;
                }
            }
        }

        match &merge {
            Some((range_a, range_b)) => {
                let min = *range_a.start().min(range_b.start());
                let max = *range_a.end().max(range_b.end());

                ranges.retain(|test| test != range_a && test != range_b);

                ranges.push(min..=max);
            }
            None => break,
        }
    }

    let res = ranges.iter().fold(0_u64, |prev, range| {
        let diff = range.end().abs_diff(*range.start()) + 1;
        prev + diff
    });

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let simple_result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(simple_result, Some(2157612992429));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
