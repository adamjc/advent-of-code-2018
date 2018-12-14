#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::boxed::Box;
use std::process;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let ops = make_ops();

    let mut freqs_vec: Vec<&str> = contents.split("\n").collect();
    freqs_vec.retain(|x| !x.is_empty());

    let mut dupe_freq = false;
    let mut current_freq = 0;
    let mut found_freqs: HashMap<i32, i32> = HashMap::new();

    while !dupe_freq {
      for freq in &freqs_vec {
        let (operator, frequency) = freq.split_at(1);
        let func = ops.get(operator).unwrap_or_else(|| process::exit(1));

        current_freq = func(current_freq, frequency.parse::<i32>().unwrap());
        dupe_freq = found_freqs.insert(current_freq, current_freq).is_some();

        if dupe_freq {
          break;
        }
      }
    }

    println!("{:?}", current_freq);

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