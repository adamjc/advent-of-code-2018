use std::io::{Read, Result, stdin};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let x = buffer.split("\n")
                  .map(|id| find_exacts(id))
                  .collect::<Vec<(i32, i32)>>()
                  .into_iter()
                  .fold(vec![0, 2], |acc, next| {
                    0
                  });

    Ok(())
}

// find any digits that match exactly twice or thrice
fn find_exacts (id: &str) -> (i32, i32) {
  let mut twice = 0;
  let mut thrice = 0;
  let mut char_counts = vec![0; 26];

  for chr in id.chars() {
    let index = (chr.to_digit(36).unwrap() as usize) - 10;
    let count: i32 = *char_counts.get(index).unwrap();
    char_counts.insert(index, count + 1);
  }

  for count in char_counts {
    if count == 2 {
      twice = 1;
    } 

    if count == 3 {
      thrice = 1;
    }
  }

  (twice, thrice)
}