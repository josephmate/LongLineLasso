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
    after_capacity: usize,
    is_ascii: bool,
    _replace: Option<&str>,
    _prepend: Option<&str>,
    _append: Option<&str>
) {
  let stdin = io::stdin();
  let mut handler = stdin.lock();
  let mut char_iter  = get_iterator(&mut handler, is_ascii);
  for match_result in match_iterator(& mut char_iter, pattern.to_string(), before_capacity, after_capacity) {
    match match_result {
      MatchResult::Match(before, found_match, after) => {
        println!("{}{}{}", before, found_match, after);
      }
      MatchResult::Unmatched(_unmatched_char) => {}
    }
  }
}

fn main() {
  let matches = App::new("LongLingLasso")
    .version("0.0.1")
    .about("Control gigantic lines. Print matches. Replace matches. Append after matches.")
    .author("Joseph Mate")
    .arg(Arg::new("pattern")
      .short('p')
      .long("pattern")
      .value_name("STRING")
      .about("The string to find matches for. Regexes are not supported")
      .takes_value(true)
    )
    .arg(Arg::new("before")
      .short('b')
      .long("before")
      .value_name("INTEGER greater than or equal to 0")
      .about("Number of characters to show before the match. Set to 0 if --replace or --append provided.")
      .takes_value(true)
    )
    .arg(Arg::new("after")
      .short('a')
      .long("after")
      .value_name("INTEGER greater than or equal to 0")
      .about("Number of characters to show after the match. Defaults to 128. Set to 0 if --replace or --append provided.")
      .takes_value(true)
    )
    .arg(Arg::new("replace")
      .long("replace")
      .value_name("STRING")
      .about("Matches found will be replaced with this string.")
      .takes_value(true)
    )
    .arg(Arg::new("append")
      .long("append")
      .value_name("STRING")
      .about("This string will be appended after all occurences of your pattern.")
      .takes_value(true)
    )
    .arg(Arg::new("prepend")
      .long("prepend")
      .value_name("STRING")
      .about("This string will be prepended before all occurences of your pattern.")
      .takes_value(true)
    )
    .arg(Arg::new("ascii")
      .long("ascii")
      .about("Assume input is ascii for improved performance")
    )
    .get_matches();
    let disable_context = matches.is_present("replace") || matches.is_present("append");
    let before = if disable_context {
      0
    } else {
      match matches.value_of("before"){
        Some(before_str) => before_str.parse::<usize>().expect("--before must be an INTEGER >= 0"),
        None => 128,
      }
    };
    let after = if disable_context {
      0
    } else {
      match matches.value_of("after"){
        Some(after_str) => after_str.parse::<usize>().expect("--after must be an INTEGER >= 0"),
        None => 128,
      }
    };
    let is_ascii  = matches.is_present("ascii");
    find_match_std_io(
      matches.value_of("pattern").expect("--pattern must be provided"),
      before,
      after,
      is_ascii,
      matches.value_of("replace"),
      matches.value_of("prepend"),
      matches.value_of("append")
    );
}

