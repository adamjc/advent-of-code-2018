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