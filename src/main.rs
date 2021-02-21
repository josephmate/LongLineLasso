use clap::{App, Arg}; 
use std::io;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::VecDeque;
use utf8_chars::{BufReadCharsExt};

struct MatchIterator<'a> {
  char_iter: &'a mut dyn Iterator<Item=char>,
  pattern: String,
  buffer_capacity: usize,
  before_capacity: usize,
  before_buffer: VecDeque<char>,
  buffer: VecDeque<char>
}

fn matchIterator<'a>(
  char_iter: &'a mut dyn Iterator<Item=char>,
  pattern: String,
  before_capacity: usize,
  after: usize
)-> MatchIterator<'a> {
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

  return MatchIterator {
    char_iter: char_iter,
    pattern: pattern,
    buffer_capacity: buffer_capacity,
    before_capacity: before_capacity,
    before_buffer: before_buffer,
    buffer: buffer
  };
}

impl Iterator for MatchIterator<'_> {
  type Item = String;

  fn next(&mut self) -> Option<String> {
    // 1. process the current character
    // 2. remove the character from the buffer and place it into the before_buffer
    // 3. if the before buffer is filled, remove the oldest character form the before_buffer

    while let Some(c) = self.char_iter.next() {
      // get the characters from the buffer to compare with the pattern
      let mut comparee = String::new();
      for i in 0 .. self.pattern.len() {
        match self.buffer.pop_front() {
          Some(j) =>  comparee.push(j),
          None => break,
        }
      }

      // found a match, record it to send later
      let current_result = if comparee == self.pattern {
        let mut result = self.before_buffer.iter().collect::<String>();
        result.push_str(&comparee);
        result.push_str(&self.buffer.iter().collect::<String>());
        Some(result)
      } else {
        None
      };

      // put all the characters into the buffer except the first one
      let mut put_back_iter = comparee.chars().rev();
      for i in put_back_iter {
        self.buffer.push_front(i);
      }
      if self.before_buffer.len() >= self.before_capacity {
        self.before_buffer.pop_front();
      }
      // at this point buffer is at buffer_capacity

      let leaving_buffer = self.buffer.pop_front().unwrap();
      // buffer is at buffer_capacity - 1 
      if self.before_buffer.len() < self.before_capacity {
        self.before_buffer.push_back(leaving_buffer);
      }

      // add another character so that we have buffer_capacity again
      self.buffer.push_back(c);

      // found a match, return it
      if current_result.is_some() {
        return current_result;
      }
    }

    // process the remaining characters in the buffer
    // can exit early if the buffer is smaller than the pattern
    while self.buffer.len() >= self.pattern.len() {
      // get the characters from the buffer to compare with the pattern
      let mut comparee = String::new();
      for i in 0 .. self.pattern.len() {
        match self.buffer.pop_front() {
          Some(j) =>  comparee.push(j),
          None => break,
        }
      }

      // found a match, record it to send later
      let current_result = if comparee == self.pattern {
        let mut result = self.before_buffer.iter().collect::<String>();
        result.push_str(&comparee);
        result.push_str(&self.buffer.iter().collect::<String>());
        Some(result)
      } else {
        None
      };

      // put all the characters into the buffer except the first one
      let mut put_back_iter = comparee.chars().rev();
      for i in put_back_iter {
        self.buffer.push_front(i);
      }
      if self.before_buffer.len() >= self.before_capacity {
        self.before_buffer.pop_front();
      }
      let leaving_buffer = self.buffer.pop_front().unwrap();
      // buffer is at buffer_capacity - 1 
      if self.before_buffer.len() < self.before_capacity {
        self.before_buffer.push_back(leaving_buffer);
      }
      
      // found a match, return it
      if current_result.is_some() {
        return current_result;
      }
    }

    return None;
  }
}

fn find_match_std_io<'a> (pattern: &'a str, before_capacity: usize, after: usize) {
  let mut stdin = io::stdin();
  let mut handler = stdin.lock();
  let mut char_iter = handler.chars().map(|x| x.unwrap());
  for found_match in matchIterator(& mut char_iter, pattern.to_string(), before_capacity, after) {
    println!("{}", found_match);
  }
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty() {
      let input = &mut "".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 0);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_one_before() {
      let input = &mut "".chars();
      let mut result = matchIterator(input, "abc".to_string(), 1, 0);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_multi_before() {
      let input = &mut "".chars();
      let mut result = matchIterator(input, "abc".to_string(), 3, 0);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_one_after() {
      let input = &mut "".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 1);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_multi_after() {
      let input = &mut "".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 3);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_one_both() {
      let input = &mut "".chars();
      let mut result = matchIterator(input, "abc".to_string(), 1, 1);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_multi_both() {
      let input = &mut "".chars();
      let mut result = matchIterator(input, "abc".to_string(), 3, 3);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match() {
      let input = &mut "abc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 0);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_one_before() {
      let input = &mut "abc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 1, 0);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multi_before() {
      let input = &mut "abc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 3, 0);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_one_after() {
      let input = &mut "abc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 1);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multi_after() {
      let input = &mut "abc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 3);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_one_both() {
      let input = &mut "abc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 1, 1);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multi_both() {
      let input = &mut "abc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 3, 3);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multiple() {
      let input = &mut "abcabc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 0);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_one_before() {
      let input = &mut "abcabc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 1, 0);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), Some("cabc".to_string()));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_multi_before() {
      let input = &mut "abcabc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 3, 0);
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), Some("abcabc".to_string()));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_one_after() {
      let input = &mut "abcabc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 1);
      assert_eq!(result.next(), Some("abca".to_string()));
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_multi_after() {
      let input = &mut "abcabc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 0, 3);
      assert_eq!(result.next(), Some("abcabc".to_string()));
      assert_eq!(result.next(), Some("abc".to_string()));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_one_both() {
      let input = &mut "abcabc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 1, 1);
      assert_eq!(result.next(), Some("abca".to_string()));
      assert_eq!(result.next(), Some("cabc".to_string()));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_multi_both() {
      let input = &mut "abcabc".chars();
      let mut result = matchIterator(input, "abc".to_string(), 3, 3);
      assert_eq!(result.next(), Some("abcabc".to_string()));
      assert_eq!(result.next(), Some("abcabc".to_string()));
      assert_eq!(result.next(), None);
    }
}
