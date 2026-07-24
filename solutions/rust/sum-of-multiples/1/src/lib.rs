use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&num| num != 0)
        .flat_map(|&num| (num..limit).step_by(num as usize))
        .collect::<HashSet<u32>>()
        .into_iter()
        .sum()
}
