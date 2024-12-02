use std::io::{self, prelude::*, BufReader};
use std::collections::BinaryHeap;

fn main() {
  let reader = BufReader::new(io::stdin());

  let mut left_heap = BinaryHeap::new();
  let mut right_heap = BinaryHeap::new();

  for line in reader.lines() {
    let s = line.unwrap();
    let nums: Vec<&str> = s.splitn(2, "   ").collect();
    let left: u32 = nums[0].parse().unwrap();
    let right: u32 = nums[1].parse().unwrap();

    left_heap.push(left);
    right_heap.push(right);
  }

  let left_vec = left_heap.into_sorted_vec();
  let right_vec = right_heap.into_sorted_vec();

  let mut total_distance = 0u32;
  for i in 0..left_vec.len() {
    total_distance += left_vec[i].abs_diff(right_vec[i]);
  }

  println!("{}", total_distance);
}
