use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn dist(a: &Vec<u64>, b: &Vec<u64>) -> u64 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a.abs_diff(*b)).pow(2))
        .sum::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_o(input, 1000)
}

pub fn part_one_o(input: &str, amount: usize) -> Option<u64> {
    let points = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| {
            e.split(",")
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut unique_pairs: HashMap<(&Vec<u64>, &Vec<u64>), u64> = HashMap::new();

    for a in points.iter() {
        let sorted_points = points
            .iter()
            .filter(|&test| test != a)
            .map(|e| (e, dist(e, a)));

        for min in sorted_points {
            let pair_a = (a, min.0);
            let pair_b = (min.0, a);

            if unique_pairs.contains_key(&pair_b) {
                continue;
            }

            unique_pairs.entry(pair_a).or_insert(min.1);
        }
    }

    let mut unique_pairs = unique_pairs
        .iter()
        .map(|e: (&(&Vec<u64>, &Vec<u64>), &u64)| ((e.0.0.clone(), e.0.1.clone()), *e.1))
        .collect::<Vec<((Vec<u64>, Vec<u64>), u64)>>();

    unique_pairs.sort_by(|a, b| a.1.cmp(&b.1));

    let mut collected: Vec<HashSet<&Vec<u64>>> = Vec::new();

    points.iter().for_each(|e| {
        let mut hs = HashSet::new();
        hs.insert(e);
        collected.push(hs);
    });

    unique_pairs.iter().take(amount).for_each(|e| {
        let a = collected
            .iter()
            .enumerate()
            .find(|test| test.1.contains(&e.0.0))
            .map(|e| e.0)
            .unwrap();
        let b = collected
            .iter()
            .enumerate()
            .find(|test| test.1.contains(&e.0.1))
            .map(|e| e.0)
            .unwrap();

        if a == b {
            return;
        }

        let min = a.min(b);
        let max = a.max(b);

        let mut merge_side = collected.swap_remove(max);
        collected[min].extend(merge_side.drain());
    });

    collected.sort_by(|a, b| b.len().cmp(&a.len()));

    let res = collected
        .iter()
        .take(3)
        .map(|e| e.len() as u64)
        .product::<u64>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input
        .split("\n")
        .filter(|test| !test.is_empty())
        .map(|e| {
            e.split(",")
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut unique_pairs: HashMap<(&Vec<u64>, &Vec<u64>), u64> = HashMap::new();

    for a in points.iter() {
        let sorted_points = points
            .iter()
            .filter(|&test| test != a)
            .map(|e| (e, dist(e, a)));

        for min in sorted_points {
            let pair_a = (a, min.0);
            let pair_b = (min.0, a);

            if unique_pairs.contains_key(&pair_b) {
                continue;
            }

            unique_pairs.entry(pair_a).or_insert(min.1);
        }
    }

    let mut unique_pairs = unique_pairs
        .iter()
        .map(|e: (&(&Vec<u64>, &Vec<u64>), &u64)| ((e.0.0.clone(), e.0.1.clone()), *e.1))
        .collect::<Vec<((Vec<u64>, Vec<u64>), u64)>>();

    unique_pairs.sort_by(|a, b| a.1.cmp(&b.1));

    let mut collected: Vec<HashSet<&Vec<u64>>> = Vec::new();

    points.iter().for_each(|e| {
        let mut hs = HashSet::new();
        hs.insert(e);
        collected.push(hs);
    });

    for e in unique_pairs.iter() {
        let a = collected
            .iter()
            .enumerate()
            .find(|test| test.1.contains(&e.0.0))
            .map(|e| e.0)
            .unwrap();
        let b = collected
            .iter()
            .enumerate()
            .find(|test| test.1.contains(&e.0.1))
            .map(|e| e.0)
            .unwrap();

        if a == b {
            continue;
        }

        let min = a.min(b);
        let max = a.max(b);

        let mut merge_side = collected.swap_remove(max);
        collected[min].extend(merge_side.drain());

        if collected.len() > 1 {
            continue;
        }

        return Some(e.0.0[0] * e.0.1[0]);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_o(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
