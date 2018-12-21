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
  let mut almost_dupes = ("", "");

  for id in &ids {  
    for next_id in &ids {
      let is_almost_dupe = id.is_almost_duplicate(next_id);

      if is_almost_dupe {
        almost_dupes = (id, next_id);
        break
      }
    }
  }

  println!("{:?}", remove_dupes(almost_dupes.0, almost_dupes.1));

  Ok(())
}

fn remove_dupes(first: &str, second: &str) -> String {
  let mut dupe_chars = String::new();

  for (i, _) in first.char_indices() {
    let this_char = first.get(i..i + 1);
      let other_char = second.get(i..i + 1);
      
      if this_char == other_char {
        dupe_chars.push_str(this_char.unwrap());
      }
  }

  dupe_chars
}