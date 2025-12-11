use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .replace("\n", "")
        .split(",")
        .filter(|test| !test.is_empty())
        .map(|e| {
            let mut e = e.split("-");
            let a = e.next().unwrap().parse::<u64>().unwrap();
            let b = e.next().unwrap().parse::<u64>().unwrap();
            (a, b)
        })
        .fold(0_u64, |mut sum_illegal_ids, (a, b)| {
            let ad = a.ilog10() + 1;
            let bd = b.ilog10() + 1;

            let start = {
                let div = 10_u64.pow(ad / 2);
                if ad % 2 == 0 { a / div } else { div / 10 }
            }
            .max(1);

            let max = {
                let div = 10_u64.pow(bd / 2);
                if bd % 2 == 0 { b / div } else { div }
            }
            .max(1);

            for i in start..=max {
                let digits = i.ilog10() + 1;
                let x = i * 10u64.pow(digits) + i;

                if a <= x && b >= x {
                    sum_illegal_ids += x;
                }
            }

            sum_illegal_ids
        });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .replace("\n", "")
        .split(",")
        .filter(|test| !test.is_empty())
        .map(|e| {
            let mut e = e.split("-");
            let a = e.next().unwrap().parse::<u64>().unwrap();
            let b = e.next().unwrap().parse::<u64>().unwrap();
            (a, b)
        })
        .fold(0_u64, |sum_illegal_ids, (a, b)| {
            let bd = b.ilog10() + 1;

            let max = {
                let div = 10_u64.pow(bd / 2);
                if bd % 2 == 0 { b / div } else { div }
            }
            .max(1);

            let mut found = HashSet::<u64>::new();

            for i in 1..=max {
                let digits = i.ilog10() + 1;

                let mut y: u32 = 0;
                let mut combine: u64 = 0;

                loop {
                    let max_length = digits * y;
                    y += 1;

                    if max_length > bd {
                        break;
                    }

                    combine += i * 10u64.pow(max_length);

                    if a <= combine && b >= combine && max_length != 0 {
                        found.insert(combine);
                    }
                }
            }

            sum_illegal_ids + found.iter().sum::<u64>()
        });

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
