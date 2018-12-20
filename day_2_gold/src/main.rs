use std::io::{Read, Result, stdin};

fn main() -> Result<()> {
  let mut ids = String::new();
  stdin().read_to_string(&mut ids)?;

  for id in ids.split("\n") {
    let (first, rest) = ids.split("\n").collect::<Vec<&str>>().split_first().unwrap();

    for next_id in rest {
      let diffs = 0;
      let next_chars = next_id.chars();
      let id_chars = id.chars();

      for (i, val) in next_id.enumerate() {
        if id_chars[i] != next_chars[i] {
          diffs += 1;
        }
      }
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
