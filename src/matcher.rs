use std::collections::VecDeque;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MatchResult {
  Unmatched(char),
  Match(String, String, String)
}

pub struct MatchIterator<'a> {
  char_iter: &'a mut dyn Iterator<Item=char>,
  pattern: String,
  before_capacity: usize,
  before_buffer: VecDeque<char>,
  buffer: VecDeque<char>,
  comparison_buffer: VecDeque<char>
}

pub fn match_iterator<'a>(
  char_iter: &'a mut dyn Iterator<Item=char>,
  pattern: String,
  before_capacity: usize,
  after: usize
)-> MatchIterator<'a> {
  let buffer_capacity = after + pattern.len();
  let before_buffer: VecDeque<char> = VecDeque::with_capacity(before_capacity);
  let mut buffer: VecDeque<char> = VecDeque::with_capacity(buffer_capacity);
  let comparison_buffer: VecDeque<char> = VecDeque::with_capacity(buffer_capacity);

  // fill the buffer so there's enough to print the after characters
  for _ in 0 .. buffer_capacity {
    match char_iter.next() {
      Some(c) => buffer.push_back(c),
      None => break,
    }
  }

  return MatchIterator {
    char_iter: char_iter,
    pattern: pattern,
    before_capacity: before_capacity,
    before_buffer: before_buffer,
    buffer: buffer,
    comparison_buffer: comparison_buffer
  };
}

impl MatchIterator<'_> {
    
  fn advance_char(&mut self, char_to_add: Option<char>) -> MatchResult {
    println!("DEBUG\n\t{}\n\t{}",
      self.before_buffer.iter().collect::<String>(),
      self.buffer.iter().collect::<String>()
    );

    // 1. process the current character
    // 2. remove the character from the buffer and place it into the before_buffer
    // 3. if the before buffer is filled, remove the oldest character form the before_buffer

    // get the characters from the buffer to compare with the pattern
    let mut matched = true;
    for pattern_char in self.pattern.chars() {
      match self.buffer.pop_front() {
        Some(input_char) => {
          self.comparison_buffer.push_back(input_char);
          if pattern_char != input_char {
            matched = false; // character didn't match
            break;
          }
        },
        None => {
          matched = false; // too short
          break;
        },
      }
    }

    // found a match, record it to send later
    if matched {
      let result = MatchResult::Match(
        self.before_buffer.iter().collect::<String>(),
        self.comparison_buffer.iter().collect::<String>(),
        self.buffer.iter().collect::<String>()
      );

      // put all the matched characters into the before_buffer
      let characters_lost_from_buffer = self.comparison_buffer.len();
      while let Some(transfer_to_before_char) = self.comparison_buffer.pop_front() {
        // make sure we don't overfill the before buffer
        if self.before_buffer.len() >= self.before_capacity {
          self.before_buffer.pop_front();
        }
        // before buffer might have 0 capacity so we need this extra check
        if self.before_buffer.len() < self.before_capacity {
          self.before_buffer.push_back(transfer_to_before_char);
        }
      }

      // add another character so that we have buffer_capacity again
      if char_to_add.is_some() {
        self.buffer.push_back(char_to_add.unwrap());
      }

      // buffer lost len(comparison_buffer)-1 characters
      // need to refill it if available
      for _ in 0..characters_lost_from_buffer {
        match self.char_iter.next() {
          Some(c) => {self.buffer.push_back(c);}
          None => {break;}
        }
      }

      return result;
    } else {
      // put all the characters into the buffer except the first one
      while let Some(put_back_char) = self.comparison_buffer.pop_back() {
        self.buffer.push_front(put_back_char);
      }
      // make sure we don't overfill the before buffer
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
      if char_to_add.is_some() {
        self.buffer.push_back(char_to_add.unwrap());
      }
      
      return MatchResult::Unmatched(leaving_buffer);
    };
  }

}

impl Iterator for MatchIterator<'_> {
  type Item = MatchResult;

  fn next(&mut self) -> Option<MatchResult> {
    
    match self.char_iter.next() {
      Some(c) => {
        return Some(self.advance_char(Some(c)));
      }
      None => {}
    }

    // process the remaining characters in the buffer
    // can exit early if the buffer is smaller than the pattern
    if self.buffer.len() >= self.pattern.len() {
      return Some(self.advance_char(None));
    } else {
      return None;
    }
  }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty() {
      let input = &mut "".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 0);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_one_before() {
      let input = &mut "".chars();
      let mut result = match_iterator(input, "abc".to_string(), 1, 0);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_multi_before() {
      let input = &mut "".chars();
      let mut result = match_iterator(input, "abc".to_string(), 3, 0);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_one_after() {
      let input = &mut "".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 1);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_multi_after() {
      let input = &mut "".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 3);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_one_both() {
      let input = &mut "".chars();
      let mut result = match_iterator(input, "abc".to_string(), 1, 1);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_empty_multi_both() {
      let input = &mut "".chars();
      let mut result = match_iterator(input, "abc".to_string(), 3, 3);
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match() {
      let input = &mut "abc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 0);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_one_before() {
      let input = &mut "abc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 1, 0);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multi_before() {
      let input = &mut "abc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 3, 0);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_one_after() {
      let input = &mut "abc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 1);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multi_after() {
      let input = &mut "abc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 3);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_one_both() {
      let input = &mut "abc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 1, 1);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multi_both() {
      let input = &mut "abc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 3, 3);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }

    #[test]
    fn test_only_match_multiple() {
      let input = &mut "abcabc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 0);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_one_before() {
      let input = &mut "abcabc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 1, 0);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), Some(MatchResult::Match("c".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_multi_before() {
      let input = &mut "abcabc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 3, 0);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), Some(MatchResult::Match("abc".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_one_after() {
      let input = &mut "abcabc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 1);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "a".to_string())));
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_multi_after() {
      let input = &mut "abcabc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 0, 3);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "abc".to_string())));
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_one_both() {
      let input = &mut "abcabc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 1, 1);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "a".to_string())));
      assert_eq!(result.next(), Some(MatchResult::Match("c".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }
    
    #[test]
    fn test_only_match_multiple_multi_both() {
      let input = &mut "abcabc".chars();
      let mut result = match_iterator(input, "abc".to_string(), 3, 3);
      assert_eq!(result.next(), Some(MatchResult::Match("".to_string(), "abc".to_string(), "abc".to_string())));
      assert_eq!(result.next(), Some(MatchResult::Match("abc".to_string(), "abc".to_string(), "".to_string())));
      assert_eq!(result.next(), None);
    }
}

