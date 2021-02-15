use clap::{App, Arg}; 
use std::io;
use std::io::BufRead;
use std::vec::Vec;
use utf8_chars::{BufReadCharsExt};


fn find_match(pattern: &str) {
  for c in io::stdin().lock().chars().map(|x| x.unwrap()) {
    println!("{}", c);
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
    .get_matches();
    
    find_match(matches.value_of("pattern").expect("--pattern must be provided"));
}
