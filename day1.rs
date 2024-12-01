use std::io::{self, prelude::*, BufReader};

fn insert_sorted<T: Ord>(vect: &mut Vec<T>, val: T) {
  vect.insert(vect.binary_search(&val).unwrap_or_else(|e| e), val);
}

fn main() {
  let reader = BufReader::new(io::stdin());

  let mut lefts = Vec::<u32>::new();
  let mut rights = Vec::<u32>::new();

  for line in reader.lines() {
    let s = line.unwrap();
    let nums: Vec<&str> = s.splitn(2, "   ").collect();
    let left: u32 = nums[0].parse().unwrap();
    let right: u32 = nums[1].parse().unwrap();

    insert_sorted(&mut lefts, left);
    insert_sorted(&mut rights, right);
  }

  let mut total_distance = 0u32;
  for i in 0..lefts.len() {
    total_distance += lefts[i].abs_diff(rights[i]);
  }

  println!("{}", total_distance);
}
