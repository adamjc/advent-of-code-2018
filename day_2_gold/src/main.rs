use std::io::{Read, Result, stdin};

trait Duplicate {
  fn is_almost_duplicate(&self, other: &str) -> bool;
}

impl Duplicate for &str {
  // -> true when exactly one character difference between strings, in exact index
  fn is_almost_duplicate(&self, other: &str) -> bool {
    let mut number_of_dupes = other.len();

    for (i, _) in self.char_indices() {
      let other_char = other.get(i..i + 1);
      let this_char = self.get(i..i + 1);

      if this_char != other_char {
        number_of_dupes -= 1;
      }
    }

    println!("self: {:?}", self);
    println!("other: {:?}", other);

    if number_of_dupes == other.len() - 1 {
      return true
    }

    false
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

      let is_almost_dupe = next_id.is_almost_duplicate(first);

      if is_almost_dupe {
        println!("{:?}", next_id);
        println!("{:?}", first);
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
