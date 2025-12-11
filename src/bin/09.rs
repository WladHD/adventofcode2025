advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| {
            e.split(",")
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let max_area = points
        .iter()
        .enumerate()
        .filter_map(|(skip_amount, a)| {
            let area = points
                .iter()
                .skip(skip_amount + 1)
                .map(|test| {
                    let h = a[0].abs_diff(test[0]) + 1;
                    let w = a[1].abs_diff(test[1]) + 1;

                    (test, h * w)
                })
                .max_by(|a, b| a.1.cmp(&b.1));

            match area {
                Some(area) => Some((a, area.0, area.1)),
                None => None,
            }
        })
        .max_by(|a, b| a.2.cmp(&b.2));

    if let Some(max_area) = max_area {
        return Some(max_area.2);
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
