use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<&[u8]> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes())
        .collect();

    let max_width = input[0].len();
    let mut beam_index = input[0].to_vec();
    let mut splits = 0_u64;

    for row in input.iter().skip(1) {
        for col in 0..max_width {
            if beam_index[col] == b'|' || beam_index[col] == b'S' {
                if row[col] == b'^' {
                    splits += 1;
                    beam_index[col] = b'.';

                    if col > 0 && beam_index[col - 1] != b'^' {
                        beam_index[col - 1] = b'|';
                    }

                    if col + 1 < max_width && beam_index[col + 1] != b'^' {
                        beam_index[col + 1] = b'|';
                    }
                }
            }
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Vec<&[u8]> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes())
        .collect();

    let max_width = input[0].len();

    let mut beam_index_count: HashMap<usize, u64> = HashMap::new();

    for (i, &c) in input[0].iter().enumerate() {
        if c == b'|' || c == b'S' {
            *beam_index_count.entry(i).or_insert(0) += 1;
        }
    }

    let mut routes: u64 = beam_index_count.values().sum();

    for row in input.iter().skip(1) {
        let mut next_beams: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in &beam_index_count {
            match row[col] {
                b'^' => {
                    if col > 0 {
                        *next_beams.entry(col - 1).or_insert(0) += count;
                        routes += count;
                    }
                    if col + 1 < max_width {
                        *next_beams.entry(col + 1).or_insert(0) += count;
                        routes += count;
                    }
                    routes -= count;
                }
                _ => {
                    *next_beams.entry(col).or_insert(0) += count;
                }
            }
        }

        beam_index_count = next_beams;
    }

    Some(routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
