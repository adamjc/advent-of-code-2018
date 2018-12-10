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
    let freqs = freqs_vec.into_iter();

    let final_freq = freqs.fold(0, |acc, freq| {
      let (operator, frequency) = freq.split_at(1);
      let func = ops.get(operator).unwrap_or_else(|| process::exit(1));

      return func(acc, frequency.parse::<i32>().unwrap());
    });

    println!("{:?}", final_freq);

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