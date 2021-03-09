use clap::{App, Arg}; 
use std::io;
use utf8_chars::{BufReadCharsExt};
use std::io::BufRead;
use lll::matcher::*;

struct CharIterator<'a> {
    buf_read: &'a mut dyn BufRead
}

fn char_iterator<'a>(
    buf_read: &'a mut dyn BufRead
) -> Box<dyn Iterator<Item=char> + 'a> {
    Box::new(CharIterator {
        buf_read: buf_read
    })
}

fn utf_iterator<'a>(
    buf_read: &'a mut dyn BufRead
) -> Box<dyn Iterator<Item=char> + 'a> {
    Box::new(buf_read.chars().map(|x| x.unwrap()))
}

fn get_iterator<'a>(
    buf_read: &'a mut dyn BufRead,
    is_ascii: bool
) -> Box<dyn Iterator<Item=char> + 'a> {
  if is_ascii {
      char_iterator(buf_read)
  } else {
      utf_iterator(buf_read)
  }
}

impl Iterator for CharIterator<'_> {
  type Item = char;

  fn next(&mut self) -> Option<char> {
    let mut buf = [0];
    match self.buf_read.read(&mut buf) {
        Ok(bytes_read) =>
            if bytes_read == 0 {
                None
            } else {
                Some(buf[0] as char)
            },
        Err(_) => None
    }
  }
}


fn find_match_std_io<'a> (
    pattern: &'a str,
    before_capacity: usize,
    after: usize,
    is_ascii: bool
) {
  let stdin = io::stdin();
  let mut handler = stdin.lock();
  let mut char_iter  = get_iterator(&mut handler, is_ascii);
  for (before, found_match, after) in match_iterator(& mut char_iter, pattern.to_string(), before_capacity, after) {
    println!("{}{}{}", before, found_match, after);
  }
}

fn main() {
  let matches = App::new("LongLingLasso")
    .version("0.0.1")
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
    .arg(Arg::new("ascii")
      .long("ascii")
      .about("Assume input is ascii for improved performance")
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
    let is_ascii  = matches.is_present("ascii");
    find_match_std_io(
      matches.value_of("pattern").expect("--pattern must be provided"),
      before,
      after,
      is_ascii
    );
}

