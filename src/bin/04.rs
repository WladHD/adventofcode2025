advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let movable_papers = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|a| a.as_bytes())
        .collect::<Vec<&[u8]>>();

    let max_height = movable_papers.len();
    let max_width = movable_papers[0].len();

    let paper = b'@';

    let mut movable: u64 = 0;

    for height in 0..max_height {
        for width in 0..max_width {
            if &movable_papers[height][width] != &paper {
                continue;
            }

            let mut x: u8 = 0;

            for rel_h in -1_i32..=1 {
                let resolved_relative_h = height as i32 + rel_h;

                if resolved_relative_h < 0 || resolved_relative_h >= max_height as i32 {
                    continue;
                }

                for rel_w in -1_i32..=1 {
                    if rel_h == 0 && rel_w == 0 {
                        continue;
                    }

                    let resolved_relative_w = width as i32 + rel_w;
                    if resolved_relative_w < 0 || resolved_relative_w >= max_width as i32 {
                        continue;
                    }

                    if &movable_papers[resolved_relative_h as usize][resolved_relative_w as usize]
                        == &paper
                    {
                        x += 1;
                    }
                }
            }

            if x < 4 {
                movable += 1;
            }
        }
    }

    Some(movable)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut movable_papers = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|a| a.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let max_height = movable_papers.len();
    let max_width = movable_papers[0].len();

    let paper = b'@';
    let empty = b'.';

    let mut movable: u64 = 0;

    loop {
        let mut cloned_papers = movable_papers.clone();
        let prev = movable;

        for height in 0..max_height {
            for width in 0..max_width {
                if &movable_papers[height][width] != &paper {
                    continue;
                }

                let mut x: u8 = 0;

                for rel_h in -1_i32..=1 {
                    let resolved_relative_h = height as i32 + rel_h;

                    if resolved_relative_h < 0 || resolved_relative_h >= max_height as i32 {
                        continue;
                    }

                    for rel_w in -1_i32..=1 {
                        if rel_h == 0 && rel_w == 0 {
                            continue;
                        }

                        let resolved_relative_w = width as i32 + rel_w;
                        if resolved_relative_w < 0 || resolved_relative_w >= max_width as i32 {
                            continue;
                        }

                        if &movable_papers[resolved_relative_h as usize]
                            [resolved_relative_w as usize]
                            == &paper
                        {
                            x += 1;
                        }
                    }
                }

                if x < 4 {
                    movable += 1;
                    cloned_papers[height][width] = empty;
                }
            }
        }

        movable_papers = cloned_papers;

        // no more changes
        if prev == movable {
            break;
        }
    }

    Some(movable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
