use clap::{App, Arg};
use std::vec::Vec;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;
use std::io::prelude::*;
use std::fs::File;

fn calc_ascii_chars() -> Vec<char> {
  return vec![
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 'Z',
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', 
    '-', '=', '_', '+', '[', ']', '\\', '{', '}', '|', 
    ';', '\'', ':', '"', ',', '.', '/', '<', '>', '?'
  ];
}

fn generate_text(
  repeated_text_length: usize,
  rng: &mut ThreadRng
) -> String {
  let ascii_chars = calc_ascii_chars();
  let ascii_char_dist = Uniform::from(0..ascii_chars.len());
  let mut result = String::new();

  for _ in 0..repeated_text_length {
    let ascii_char_idx = ascii_char_dist.sample(rng);
    result.push(ascii_chars[ascii_char_idx]);
  }

  result
}

fn generate(
  num_chars: usize,
  repeated_text_length: usize,
  repeated_text_filepath: &str,
  num_times: usize,
  out_filepath: &str
) {
  let mut rng = rand::thread_rng();
  let repeated_text = generate_text(repeated_text_length, &mut rng);
  {
    let mut repeated_text_file = File::create(repeated_text_filepath)
      .expect("count not create --repeated_text_file");
    write!(repeated_text_file, "{}", repeated_text)
      .expect("count not write to --repeated_text_file");
  }

  let mut text_positions = HashMap::new();
  let text_position_dist = Uniform::from(0..num_chars);
  for _ in 0..num_times {
    let count = text_positions.entry(text_position_dist.sample(&mut rng)).or_insert(0);
    *count += 1;
  }

  let ascii_chars = calc_ascii_chars();
  let ascii_char_dist = Uniform::from(0..ascii_chars.len());
  let mut out_file = File::create(out_filepath)
    .expect("count not create --out file");
  for char_posn in 0..num_chars {
    if let Occupied(count) = text_positions.entry(char_posn) {
      for _ in 0 .. *count.get() {
        write!(out_file, "{}", repeated_text)
          .expect("count not write to --out file");
      }
    }

    let ascii_char_idx = ascii_char_dist.sample(&mut rng);
    write!(out_file,"{}", ascii_chars[ascii_char_idx])
      .expect("count not write to --out file");
  }
}

fn main() {
  let matches = App::new("GenerateLargeFiles")
    .version("0.0.1")
    .about("Generates large files.")
    .author("Joseph Mate")
    .arg(Arg::new("characters")
      .long("characters")
      .value_name("INTEGER greater than or equal to 0")
      .about("Number of characters to generate.")
      .takes_value(true)
      .required(true)
    )
    .arg(Arg::new("repeated_text_length")
      .long("--repeated_text_length")
      .value_name("INTEGER greater than or equal to 0")
      .about("The length of the repated text to randomly insert within the large file.")
      .takes_value(true)
      .required(true)
    )
    .arg(Arg::new("repeated_text_file")
      .long("repeated_text_file")
      .value_name("PATH")
      .about("path the path to save the repeated_text to.")
      .takes_value(true)
      .required(true)
    )
    .arg(Arg::new("times")
      .long("times")
      .value_name("INTEGER greater than or equal to 0")
      .about("The number of times to insert the repeated text of length --repeated_text_length.")
      .takes_value(true)
      .required(true)
    )
    .arg(Arg::new("out")
      .long("out")
      .value_name("PATH")
      .about("path the path to save the repeated_text to.")
      .takes_value(true)
      .required(true)
    )
    .get_matches();
    
    let num_chars = matches.value_of("characters")
      .expect("--characters must be provided")
      .parse::<usize>().expect("--characters must be an INTEGER >= 0");
    let num_times = matches.value_of("times")
      .expect("--times must be provided")
      .parse::<usize>().expect("--times must be an INTEGER >= 0");
    let repeated_text_length = matches.value_of("repeated_text_length")
      .expect("--repeated_text_length must be provided")
      .parse::<usize>().expect("--repeated_text_length must be an INTEGER >= 0");
    let repeated_text_filepath = matches.value_of("repeated_text_file")
      .expect("--repeated_text_file must be provided");
    let out_filepath = matches.value_of("out")
      .expect("--out must be provided");
    generate(num_chars, repeated_text_length, repeated_text_filepath, num_times, out_filepath);
}