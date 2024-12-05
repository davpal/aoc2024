use std::io::{self, prelude::*, BufReader};

fn main() {
  let reader = BufReader::new(io::stdin());
 
  let mut matrix: Vec<Vec<char>> = vec![];

  for line in reader.lines() {
    let line = line.unwrap();
    matrix.push(line.chars().collect());
  }

  let mut xmas_count = 0;
  for i in 0..matrix.len() {
    println!("{}", i);
    for j in 0..matrix[i].len() {
      if j+3 < matrix.len() {
        let horizontal: String = matrix[i][j..j+4].iter().collect();
        if horizontal == "XMAS" || horizontal == "SAMX" {
          xmas_count += 1;
        }
      }

      if i+3 < matrix.len() {
        let vertical: String = (0..4).map(|k| matrix[i + k][j]).collect(); 
        if vertical == "XMAS" || vertical == "SAMX" {
          xmas_count += 1;
        }
      }


      if i+3 < matrix.len() && j+3 < matrix[i].len() {
        let diagonal: String = (0..4).map(|k| matrix[i + k][j + k]).collect();
        if diagonal == "XMAS" || diagonal == "SAMX" {
          xmas_count += 1;
        }
      }

      if i+3 < matrix.len() && j >= 3 {
        let diagonal: String = (0..4).map(|k| matrix[i + k][j - k]).collect();
        if diagonal == "XMAS" || diagonal == "SAMX" {
          xmas_count += 1;
        }
      }
    }
  }

  println!("{}", xmas_count);
}
