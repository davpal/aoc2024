use std::io::{self, prelude::*, BufReader};

fn main() {
  let reader = BufReader::new(io::stdin());
  
  let mut mul_chars = 0;
  let mut arg1: String = String::from("");
  let mut arg1_started = false;
  let mut arg2: String = String::from("");
  let mut arg2_started = false;
  let mut total_sum: i64 = 0;
  
  for byte in reader.bytes() {
    let c = byte.unwrap() as char;
    
    if c == 'm' {
      mul_chars = 1;
    } else if mul_chars == 1 && c == 'u' {
      mul_chars = 2;
    } else if mul_chars == 2 && c == 'l' {
      mul_chars = 3;
    } else if mul_chars == 3 && c == '(' {
      arg1_started = true;
    } else if arg1_started && c.is_digit(10) {
      arg1.push(c);
    } else if arg1_started && c == ',' {
      arg1_started = false;
      arg2_started = true;
    } else if arg2_started && c.is_digit(10) {
      arg2.push(c);
    } else if arg2_started && c == ')' { // && arg1.len() >= 1 && arg1.len() <= 3 && arg2.len() >= 1 && arg2.len() <= 3 {
      if arg1.len() <= 3 || arg2.len() <= 3 {
        println!("mul({},{})", arg1, arg2);
        let a: i64 = arg1.parse().unwrap();
        let b: i64 = arg2.parse().unwrap();
        total_sum += a * b; 
      }

      mul_chars = 0;
      arg1_started = false;
      arg2_started = false;
      arg1.clear();
      arg2.clear();
    } else {
      mul_chars = 0;
      arg1_started = false;
      arg2_started = false;
      arg1.clear();
      arg2.clear();
    }
  }

  println!("{}", total_sum);
}
