advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, zeroes) = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| {
            let (direction, steps) = e.split_at(1);
            let steps = steps.parse::<i32>().unwrap()
                * match direction {
                    "L" => -1,
                    _ => 1,
                };

            steps
        })
        .fold(
            (50_i32, 0_u64),
            |(previous_position, previous_zeroes), steps| {
                let mut new_pos = (previous_position + steps) % 100;

                if new_pos < 0 {
                    new_pos = 100 + new_pos;
                }

                let hits = if new_pos == 0 { 1 } else { 0 };
                let zeroes = previous_zeroes + hits as u64;
                (new_pos, zeroes)
            },
        );

    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, zeroes) = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| {
            let (direction, steps) = e.split_at(1);
            let steps = steps.parse::<i32>().unwrap()
                * match direction {
                    "L" => -1,
                    _ => 1,
                };

            steps
        })
        .fold(
            (50_i32, 0_u64),
            |(previous_position, previous_zeroes), steps| {
                let mut new_pos = (previous_position + steps) % 100;

                if new_pos < 0 {
                    new_pos = 100 + new_pos;
                }

                let mut hits = steps.abs() / 100;

                if previous_position != 0
                    && (new_pos == 0
                        || steps < 0 && new_pos > previous_position
                        || steps > 0 && new_pos < previous_position)
                {
                    hits += 1;
                }

                let zeroes = previous_zeroes + hits as u64;
                (new_pos, zeroes)
            },
        );

    Some(zeroes)
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
            "examples", DAY, 4,
        ));
        assert_eq!(simple_result, Some(1));

        let simple_result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(simple_result, Some(1));

        let simple_result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(simple_result, Some(10));

        let simple_result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(simple_result, Some(10));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
