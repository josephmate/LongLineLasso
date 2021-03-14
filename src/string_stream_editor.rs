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

fn unescape_str(escaped: &str) -> String {
  return escaped.to_string()
      .replace("\\n", "\n")
      .replace("\\\\", "\\");
}

pub fn process_string_stream_bufread_bufwrite<'a> (
    bufread: &'a mut dyn BufRead,
    bufwriter: &'a mut dyn Write,
    escaped_pattern: &'a str,
    before_capacity: usize,
    after_capacity: usize,
    is_ascii: bool,
    escaped_replace: Option<&str>,
    escaped_prepend: Option<&str>,
    escaped_append: Option<&str>
) {
  // escape \n to \n
  // then escape \\ to \
  let pattern = unescape_str(escaped_pattern);
  let replace = escaped_replace.map(unescape_str);
  let prepend = escaped_prepend.map(unescape_str);
  let append = escaped_append.map(unescape_str);

  let mut char_iter  = get_iterator(bufread, is_ascii);
  if replace.is_some() || prepend.is_some() || append.is_some() {
    for match_result in match_iterator(& mut char_iter, pattern, 0, 0) {
      match match_result {
        MatchResult::Match(_, found_match, _) => {
          if prepend.is_some() {
            write!(bufwriter, "{}", prepend.as_ref().unwrap())
              .expect("Unable to prepend to bufwriter do to io error.");
          }
          if replace.is_some() {
            write!(bufwriter, "{}", replace.as_ref().unwrap())
              .expect("Unable to replace to bufwriter do to io error.");
          } else {
            write!(bufwriter, "{}", found_match)
              .expect("Unable to write to bufwriter do to io error.");
          }
          if append.is_some() {
            write!(bufwriter, "{}", append.as_ref().unwrap())
              .expect("Unable to append to bufwriter do to io error.");
          }
        }
        MatchResult::Unmatched(unmatched_char) => {
          write!(bufwriter, "{}", unmatched_char)
              .expect("Unable to write unmatched char to bufwriter do to io error.");
        }
      }
    }
  } else {
    for match_result in match_iterator(& mut char_iter, pattern, before_capacity, after_capacity) {
      match match_result {
        MatchResult::Match(before, found_match, after) => {
          writeln!(bufwriter, "{}{}{}", before, found_match, after)
              .expect("Unable to write match to bufwriter do to io error.");
        }
        MatchResult::Unmatched(_unmatched_char) => {}
      }
    }
  }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_empty() {
      let mut input = b"" as &[u8];
      let patten = "abc";
      let output = Vec::new();
      let output_buffer = BufWriter::new(output);
      process_string_stream_bufread_bufwrite(
        input,
        output,
        pattern,
        0, // before capacity
        0, // after capacity
        true, // is_ascii
        None, //replace
        None, //prepend
        Some("\\n") //append
      );
      output_buffer.flush();
      assert_eq!(output.
    }
}

