#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::boxed::Box;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let ops = make_ops();

    let freqs = contents.split("\n");

    for freq in freqs {
      let freq_str = freq.to_string();
      if freq_str.len() > 1 {
        let split_freq = freq_str.split_at(1);
        // Now have a tuple containing (<operator>, <value>)
      }
    }

    Ok(())
}

fn make_ops () -> HashMap<String, Box<Fn(i32, i32) -> i32>> {
  let mut ops: HashMap<String, Box<Fn(i32, i32) -> i32>> = HashMap::new();

  ops.insert("+".into(), Box::new(add));
  ops.insert("-".into(), Box::new(sub));

  return ops
}

fn add (x: i32, y: i32) -> i32 {
  return x + y;
}

fn sub (x: i32, y: i32) -> i32 {
  return x - y;
}