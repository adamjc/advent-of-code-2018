use std::io::{Read, Result, stdin};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let exact_digits_count = buffer.split("\n")
                  .map(|id| find_exacts(id))
                  .collect::<Vec<(i32, i32)>>()
                  .into_iter()
                  .fold(vec![0, 2], |mut acc, next| {
                    acc[0] += next.0;
                    acc[1] += next.1;
                    
                    acc
                  });

    let checksum = exact_digits_count[0] * exact_digits_count[1];

    println!("{:?}", checksum);

    Ok(())
}

// find any digits that match exactly twice or thrice
fn find_exacts (id: &str) -> (i32, i32) {
  let mut twice = 0;
  let mut thrice = 0;
  let mut char_counts = vec![0; 26];

  for chr in id.chars() {
    let index = (chr.to_digit(36).unwrap() as usize) - 10;    
    char_counts[index] += 1;
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