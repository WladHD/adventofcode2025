advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let joltage =
        input
            .split("\n")
            .filter(|test| !test.is_empty())
            .fold(0_u64, |prev_joltage, row| {
                let mut start_index = 0;
                let mut joltage: u64 = 0;

                for remaining in (0..2).rev() {
                    let (_, a) = row.split_at(start_index);
                    let (a, _) = a.split_at(a.len().saturating_sub(remaining));

                    let found = a
                        .as_bytes()
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
                        .unwrap();

                    // ascii parsing
                    joltage += 10_u64.pow(remaining as u32) * (found.1 - 48) as u64;

                    start_index += found.0 + 1;
                }

                prev_joltage + joltage
            });

    Some(joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let joltage =
        input
            .split("\n")
            .filter(|test| !test.is_empty())
            .fold(0_u64, |prev_joltage, row| {
                let mut start_index = 0;
                let mut joltage: u64 = 0;

                for remaining in (0..12).rev() {
                    let (_, a) = row.split_at(start_index);
                    let (a, _) = a.split_at(a.len().saturating_sub(remaining));

                    let found = a
                        .as_bytes()
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
                        .unwrap();

                    // ascii parsing
                    joltage += 10_u64.pow(remaining as u32) * (found.1 - 48) as u64;

                    start_index += found.0 + 1;
                }

                prev_joltage + joltage
            });

    Some(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
