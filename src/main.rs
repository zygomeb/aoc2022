
#![feature(iter_array_chunks)]

mod solutions;
mod data;
mod toy_data;

use solutions::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  
    solve14p2();

    Ok(())
}
