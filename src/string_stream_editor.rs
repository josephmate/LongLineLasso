use utf8_chars::{BufReadCharsExt};
use std::io::BufRead;
use std::io::Write;
use crate::matcher::*;

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

pub fn process_string_stream_bufread_bufwrite<'a> (
    bufread: &'a mut dyn BufRead,
    bufwrite: &'a mut dyn Write,
    pattern: &'a str,
    before_capacity: usize,
    after_capacity: usize,
    is_ascii: bool,
    replace: Option<&str>,
    prepend: Option<&str>,
    append: Option<&str>
) {
  let mut char_iter  = get_iterator(bufread, is_ascii);
  if replace.is_some() || prepend.is_some() || append.is_some() {
    // TODO
  } else {
    for match_result in match_iterator(& mut char_iter, pattern.to_string(), before_capacity, after_capacity) {
      match match_result {
        MatchResult::Match(before, found_match, after) => {
          writeln!(bufwrite, "{}{}{}", before, found_match, after);
        }
        MatchResult::Unmatched(_unmatched_char) => {}
      }
    }
  }
}
