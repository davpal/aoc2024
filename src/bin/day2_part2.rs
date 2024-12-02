use std::io::{self, prelude::*, BufReader};

fn is_report_safe(nums: &Vec<u32>) -> bool {
  let mut inc = false;
  let mut dec = false;

  for i in 1..nums.len() {
    let diff = nums[i].abs_diff(nums[i-1]);

    if diff < 1 || diff > 3 {
      return false;
    }

    if nums[i] > nums[i-1] {
      dec = true;
    } else {
      inc = true;
    }
  }

  return (inc && !dec) || (!inc && dec);
}

fn is_report_fixable(nums: &Vec<u32>) -> bool {
  for i in 0..nums.len() {
    let mut variant = nums.to_vec();
    variant.remove(i);
    if is_report_safe(&variant) {
      return true;
    }
  }

  return false;
}

fn main() {
  let reader = BufReader::new(io::stdin());

  let mut safe_count = 0;

  for line in reader.lines() {
    let s = line.unwrap();
    let nums: Vec<u32> = s.split_whitespace()
                          .map(|x| x.parse().unwrap())
                          .collect();

    if is_report_safe(&nums) || is_report_fixable(&nums) {
      safe_count += 1;
    }
  }

  println!("{}", safe_count);
}
