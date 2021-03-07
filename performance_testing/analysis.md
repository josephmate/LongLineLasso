

String::push
===========================
Flame graph shows about 1/3 of the time is spent appending strings.
Like this code code is the root cause:
```
// get the characters from the buffer to compare with the pattern
let mut comparee = String::new();
for _ in 0 .. self.pattern.len() {
  match self.buffer.pop_front() {
    Some(j) =>  comparee.push(j),
    None => break,
  }
}
```

Here I'm building up a string so that I can easily compare it to the expected pattern.
I have two options to improve the performance.
1. Instead of creating a string and doing string comparison, I can pop the
	 characters and compare them directly with the pattern.
2. I can try to re-use and clear the same string buffer so it doesn't need to
	 keep growing and shrinking.

VecDeque<T>::push_front and VecDeque<T>::pop_front
===========================
Takes about 1/2 the time.
I'm not sure how else I can improve this piece.
I need to use a cyclical buffer of characters to 
I suspect it's because I pop and push len(pattern) for every character in the input.
Hopefully by solving the String:push problem, I will also reduce this problem.
By only popping the characters I need,
until the first character does not match,
I reduce the number of characters I need to check.

Only comparing the characters needed
===================
Only comparing the characters need improved performance from 1m41.556s to 0m3.313s (30x).
This sort of makes sense since the patterns are 100 characters long.
I was expecting about a 100x improvement.

utf8_chars::BufReadCharsExt::read_char_raw
=========================================
Reading the characters takes about 50% of the time now.
I'm not sure where to go from here.
I'm assuming that the bytes are read into a buffer,
then we read the characters one by from from the buffer.
Maybe we should try reading bytes and converting them directly to characters,
assuming the are ascii to compare?
Maybe we can make it a flag like `--ascii`?

Dropping utf_chars in favour of only ascii
==========================================
Performance experiments show a drop from 34 to 24 seconds.
This improvement is much smaller than I was expecting.
By using ascii, I was hoping that I would have similar performance to grep and rg.
Looks like we need to keep digging.

std::io::buffered::BufReader<R> as std::io::Read>::read
===================
25% of the time is spent here.
I'm not sure what else I can do.
Maybe read is not buffered, even though we're using BufRead.
Maybe we need to buffer it ourselfs?
