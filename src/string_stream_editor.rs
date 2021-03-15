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

struct OutputBuffer<'a> {
    write: &'a mut dyn Write,
    buffer: &'a mut [u8],
    buffer_capacity: usize,
    buffer_used: usize
}

impl OutputBuffer<'_> {

  fn append_char(&mut self, to_add: char) -> std::io::Result<()> {
    let byte_len = to_add.len_utf8();
    if byte_len + self.buffer_used > self.buffer_capacity {
      self.flush()?;
    }
    to_add.encode_utf8(&mut self.buffer[self.buffer_used..]);
    self.buffer_used = self.buffer_used + byte_len;
    Ok(())
  }

  fn append_str(&mut self, to_add: &str) -> std::io::Result<()> {
    for b in to_add.bytes() {
      if self.buffer_used >= self.buffer_capacity {
        self.flush()?;
      }
      self.buffer[self.buffer_used] = b;
      self.buffer_used = self.buffer_used + 1;
    }
    Ok(())
  }

  fn flush(&mut self) -> std::io::Result<()> {
    if self.buffer_used > 0 {
      self.write.write_all(&self.buffer[..self.buffer_used])?;
      self.buffer_used = 0;
    }
    Ok(())
  }
}

fn unescape_str(escaped: &str) -> String {
  return escaped.to_string()
      .replace("\\n", "\n")
      .replace("\\\\", "\\");
}


// 128KB
const BUFFER_CAPACITY: usize = 1024 * 128;
pub fn process_string_stream_bufread_bufwrite<'a> (
    bufread: &'a mut dyn BufRead,
    write: &'a mut dyn Write,
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
  
  // we could make this configurable
  let mut buffer = [0; BUFFER_CAPACITY];
  let mut output_buffer = OutputBuffer {
    write: write,
    buffer: &mut buffer,
    buffer_capacity: BUFFER_CAPACITY,
    buffer_used: 0
  };

  let mut char_iter  = get_iterator(bufread, is_ascii);
  if replace.is_some() || prepend.is_some() || append.is_some() {
    for match_result in match_iterator(& mut char_iter, pattern, 0, 0) {
      match match_result {
        MatchResult::Match(_, found_match, _) => {
          if prepend.is_some() {
            output_buffer.append_str(prepend.as_ref().unwrap())
              .expect("Unable to prepend to bufwriter do to io error.");
          }
          if replace.is_some() {
            output_buffer.append_str(replace.as_ref().unwrap())
              .expect("Unable to replace to bufwriter do to io error.");
          } else {
            output_buffer.append_str(&found_match)
              .expect("Unable to write to bufwriter do to io error.");
          }
          if append.is_some() {
            output_buffer.append_str(append.as_ref().unwrap())
              .expect("Unable to append to bufwriter do to io error.");
          }
        }
        MatchResult::Unmatched(unmatched_char) => {
          output_buffer.append_char(unmatched_char).expect("Unable to write unmatched char to bufwriter do to io error.");
        }
      }
    }
  } else {
    for match_result in match_iterator(& mut char_iter, pattern, before_capacity, after_capacity) {
      match match_result {
        MatchResult::Match(before, found_match, after) => {
          output_buffer.append_str(&before).expect("Unable to write match to bufwriter do to io error.");
          output_buffer.append_str(&found_match).expect("Unable to write match to bufwriter do to io error.");
          output_buffer.append_str(&after).expect("Unable to write match to bufwriter do to io error.");
        }
        MatchResult::Unmatched(_unmatched_char) => {}
      }
    }
  }
  output_buffer.flush().expect("Unable to flush final buffer output.");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    fn test_append(
        pattern: &str,
        input: &str,
        expected: &str
    ) {
      let mut input_bytes = input.as_bytes();
      let mut output = Vec::new();
      process_string_stream_bufread_bufwrite(
        &mut input_bytes,
        &mut output,
        pattern,
        0, // before capacity
        0, // after capacity
        true, // is_ascii
        None, //replace
        None, //prepend
        Some("\\n") //append
      );
      assert_eq!(String::from_utf8(output).unwrap(),
        expected.to_string());
    }

    #[test]
    fn test_append_empty() {
      test_append(
        "abc", // pattern
        "", // input
        "" // expected
      );
    }

    #[test]
    fn test_append_once() {
      test_append(
        "abc", // pattern
        "abc", // input
        "abc\n" // expected
      );
    }

    #[test]
    fn test_append_once_unmatched_after() {
      test_append(
        "abc", // pattern
        "abcaaaaaa", // input
        "abc\naaaaaa" // expected
      );
    }

    #[test]
    fn test_append_twice() {
      test_append(
        "abc", // pattern
        "abcabc", // input
        "abc\nabc\n" // expected
      );
    }
}

