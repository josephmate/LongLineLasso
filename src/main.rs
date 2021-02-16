use clap::{App, Arg}; 
use std::io;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::VecDeque;
use utf8_chars::{BufReadCharsExt};


fn find_match(char_iter: &mut dyn Iterator<Item=char>, pattern: &str, before_capacity: usize, after: usize) {
  let buffer_capacity = after + pattern.len();
  let mut before_buffer: VecDeque<char> = VecDeque::with_capacity(before_capacity);
  let mut buffer: VecDeque<char> = VecDeque::with_capacity(buffer_capacity);

  // fill the buffer so there's enough to print the after characters
  for i in 0 .. buffer_capacity {
    match char_iter.next() {
      Some(c) => buffer.push_back(c),
      None => break,
    }
  }

  // 1. process the current character
  // 2. remove the character from the buffer and place it into the before_buffer
  // 3. if the before buffer is filled, remove the oldest character form the before_buffer
  for c in char_iter {
    // get the characters from the buffer to compare with the pattern
    let mut comparee = String::new();
    for i in 0 .. pattern.len() {
      match buffer.pop_front() {
        Some(j) =>  comparee.push(j),
        None => break,
      }
    }

    // found a match, output it
    if comparee == pattern {
      println!("{}{}{}",
        before_buffer.iter().collect::<String>(),
        comparee,
        buffer.iter().collect::<String>());
    }

    // put all the characters into the buffer except the first one
    let mut put_back_iter = comparee.chars().rev();
    for i in put_back_iter {
      buffer.push_front(i);
    }
    if before_buffer.len() >= before_capacity {
      before_buffer.pop_front();
    }
    // at this point buffer is at buffer_capacity

    before_buffer.push_back(buffer.pop_front().unwrap());
    // buffer is at buffer_capacity - 1 

    // add another character so that we have buffer_capacity again
    buffer.push_back(c);
  }

  // process the remaining characters in the buffer
  // can exit early if the buffer is smaller than the pattern
  while buffer.len() >= pattern.len() {
    // get the characters from the buffer to compare with the pattern
    let mut comparee = String::new();
    for i in 0 .. pattern.len() {
      match buffer.pop_front() {
        Some(j) =>  comparee.push(j),
        None => break,
      }
    }

    // found a match, output it
    if comparee == pattern {
      println!("{}{}{}",
        before_buffer.iter().collect::<String>(),
        comparee,
        buffer.iter().collect::<String>());
    }

    // put all the characters into the buffer except the first one
    let mut put_back_iter = comparee.chars().rev();
    for i in put_back_iter {
      buffer.push_front(i);
    }
    if before_buffer.len() >= before_capacity {
      before_buffer.pop_front();
    }
    before_buffer.push_back(buffer.pop_front().unwrap());
  }
}

fn find_match_std_io(pattern: &str, before_capacity: usize, after: usize) {
  let mut stdin = io::stdin();
  let mut handler = stdin.lock();
  let mut char_iter = handler.chars().map(|x| x.unwrap());
  find_match(&mut char_iter, pattern, before_capacity, after);
}

fn main() {
  let matches = App::new("LongLingLasso")
    .version("1.0.0")
    .about("Find multiple matches in a GIANT line.\nSplit an inconveniently HUMONGOUS line multiple times.")
    .author("Joseph Mate")
    .arg(Arg::new("pattern")
      .short('p')
      .long("pattern")
      .value_name("PATTERN")
      .about("The pattern to find matches for. Regexes are not supported")
      .takes_value(true)
      .required(true)
    )
    .arg(Arg::new("before")
      .short('b')
      .long("before")
      .value_name("INTEGER greater than or equal to 0")
      .about("Number of characters to show before the match. Defaults to 128.")
      .takes_value(true)
    )
    .arg(Arg::new("after")
      .short('a')
      .long("after")
      .value_name("INTEGER greater than or equal to 0")
      .about("Number of characters to show after the match. Defaults to 128.")
      .takes_value(true)
    )
    .get_matches();
    
    let before = match matches.value_of("before"){
      Some(before_str) => before_str.parse::<usize>().expect("--before must be an INTEGER >= 0"),
      None => 128,
    };
    let after = match matches.value_of("after"){
      Some(after_str) => after_str.parse::<usize>().expect("--after must be an INTEGER >= 0"),
      None => 128,
    };
    find_match_std_io(
      matches.value_of("pattern").expect("--pattern must be provided"),
      before,
      after
    );
}
