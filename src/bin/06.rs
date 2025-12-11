advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| {
            e.split(" ")
                .filter(|test| !test.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let max_height = input.len();
    let mut res: u64 = 0;

    for i in 0..input[0].len() {
        let sign = input[max_height - 1][i];

        let iter = (0..(max_height - 1)).map(|h| input[h][i].parse::<u64>().unwrap());

        res += match sign {
            "+" => iter.sum::<u64>(),
            "*" => iter.product::<u64>(),
            _ => todo!(),
        };
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| e.as_bytes())
        .collect::<Vec<&[u8]>>();

    let max_height = input.len();

    let mut res = 0;

    let mut iterator = (0..input[0].len()).rev();

    let mut numbers: Vec<String> = Vec::with_capacity(max_height - 2);
    (0..(max_height - 1)).for_each(|_| numbers.push("".to_string()));

    while let Some(i) = iterator.next() {
        (0..(max_height - 1)).for_each(|h| match input[h][i] {
            e if e != b' ' => {
                numbers[i % (max_height - 1)].push(e as char);
            }
            _ => {}
        });

        match input[max_height - 1][i] {
            b'*' => {
                res += numbers
                    .iter()
                    .filter(|test| !test.is_empty())
                    .map(|e| e.parse::<u64>().unwrap())
                    .product::<u64>();

                iterator.next();
                numbers.iter_mut().for_each(|e| e.clear());
            }
            b'+' => {
                res += numbers
                    .iter()
                    .filter(|test| !test.is_empty())
                    .map(|e| e.parse::<u64>().unwrap())
                    .sum::<u64>();

                iterator.next();
                numbers.iter_mut().for_each(|e| e.clear());
            }
            _ => {}
        };
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
