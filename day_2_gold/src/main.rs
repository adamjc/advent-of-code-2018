use std::io::{Read, Result, stdin};

trait Duplicate {
  fn find_dupes(&self, other: &str) -> &str;
}

impl Duplicate for &str {
  fn find_dupes(&self, other: &str) -> &str {
    // Go through each character, find how many dupes...
    "Not yet implemented"
  }
}

fn main() -> Result<()> {
  let mut buffer = String::new();
  stdin().read_to_string(&mut buffer)?;

  let ids = buffer.split("\n").collect::<Vec<&str>>();

  for id in &ids {
    let (first, rest) = ids.split_first().unwrap();

    for next_id in rest {
      let diffs = 0;
      let next_chars = next_id.chars();
      let id_chars = id.chars();

      println!("{:?}", next_id.find_dupes("Hello"));
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works () {
    let a = "abc".as_bytes().into_iter().fold("".to_string(), |mut acc, next| {
      acc.push_str(&next.to_string());

      acc
    });

    let b = "abd".as_bytes().into_iter().fold("".to_string(), |mut acc, next| {
      acc.push_str(&next.to_string());

      acc
    });

    assert_eq!(a.parse::<u32>().unwrap(), 979899);
    assert_eq!(b.parse::<u32>().unwrap(), 9798100);
    assert_eq!(202 & 202, 202);
    assert_eq!(a.parse::<u32>().unwrap() & b.parse::<u32>().unwrap(), 979899)
  }
}
