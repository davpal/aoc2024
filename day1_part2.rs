use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
  let reader = BufReader::new(io::stdin());

  let mut lefts = Vec::<u32>::new();
  let mut rights_count = HashMap::<u32, u32>::new();

  for line in reader.lines() {
    let s = line.unwrap();
    let nums: Vec<&str> = s.splitn(2, "   ").collect();
    let left: u32 = nums[0].parse().unwrap();
    let right: u32 = nums[1].parse().unwrap();

    lefts.push(left);

    rights_count.entry(right)
                .and_modify(|e| *e += 1)
                .or_insert(1);
  }

  let mut similarity = 0u32;
  for i in 0..lefts.len() {
    if let Some(v) = rights_count.get(&lefts[i]) {
      similarity += lefts[i] * v;
    }
  }

  println!("{}", similarity);
}
