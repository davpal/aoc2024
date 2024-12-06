use std::io::{self, prelude::*, BufReader};

fn main() {
  let reader = BufReader::new(io::stdin());
 
  let mut matrix: Vec<Vec<char>> = vec![];

  for line in reader.lines() {
    let line = line.unwrap();
    matrix.push(line.chars().collect());
  }

  let mut xmas_count = 0;
  for i in 1..matrix.len() - 1 {
    for j in 1..matrix[i].len() - 1 {
      let diag1: String = [
        matrix[i - 1][j - 1],
        matrix[i][j],
        matrix[i + 1][j + 1]
      ].iter().collect();

      let diag2: String = [
        matrix[i - 1][j + 1],
        matrix[i][j],
        matrix[i + 1][j - 1]
      ].iter().collect();
      println!("{} {}", diag1, diag2); 
      if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
        xmas_count += 1;
      }
    }
  }

  println!("{}", xmas_count);
}
