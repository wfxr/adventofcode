use crate::Solution;
use std::cmp::Ordering;
use std::collections::HashSet;

fn two_sum(nums: &[i64], target: i64) -> bool {
    let nums: HashSet<_> = nums.iter().collect();
    nums.iter().any(|&x| nums.contains(&(target - x)))
}

fn contiguous_sum(nums: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut sum = 0;
    let (mut min, mut max) = (i64::MAX, i64::MIN);
    for &x in nums.iter() {
        sum += x;
        min = min.min(x);
        max = max.max(x);
        match sum.cmp(&target) {
            Ordering::Equal => return Some((min, max)),
            Ordering::Greater => return None,
            _ => continue,
        }
    }
    None
}

fn p1_solve(nums: &[i64]) -> i64 {
    *nums
        .iter()
        .enumerate()
        .skip(25)
        .find(|(i, &x)| !two_sum(&nums[..*i], x))
        .unwrap()
        .1
}

fn p2_solve(nums: &[i64]) -> i64 {
    let target = p1_solve(nums);
    for i in 0..nums.len() {
        if let Some((min, max)) = contiguous_sum(&nums[i..], target) {
            return max + min;
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let input = parse_input(input);
        let result = p1_solve(&input);
        Ok(result.to_string())
    },
    part2: |input| {
        let input = parse_input(input);
        let result = p2_solve(&input);
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(1309761972, 177989832);
