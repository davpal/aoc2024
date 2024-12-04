use std::io::{self, prelude::*, BufReader};

fn main() {
  let reader = BufReader::new(io::stdin());
  
  let mut mul_chars = 0;
  let mut arg1: String = String::from("");
  let mut arg1_started = false;
  let mut arg2: String = String::from("");
  let mut arg2_started = false;
  let mut enabled = true;
  let mut total_sum: i64 = 0;
  let mut do_chars = 0;
  
  for byte in reader.bytes() {
    let c = byte.unwrap() as char;
    
    if c == 'd' {
      do_chars = 1;
    } else if do_chars == 1 && c == 'o' {
      do_chars = 2;
    } else if do_chars == 2 && c == '(' {
      do_chars = 3;
    } else if do_chars == 3 && c == ')' {
      println!("do");
      enabled = true;
      do_chars = 0;
    } else if do_chars == 2 && c == 'n' {
      println!("going don't");
      do_chars = 4;
    } else if do_chars == 4 && c == '\'' {
      do_chars = 5;
    } else if do_chars == 5 && c == 't' {
      do_chars = 6;
    } else if do_chars == 6 && c == '(' {
      do_chars = 7;
    } else if do_chars == 7 && c == ')' {
      enabled = false;
      println!("don't");
      do_chars = 0;
    } else if c == 'm' {
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
      if enabled && (arg1.len() <= 3 || arg2.len() <= 3) {
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
