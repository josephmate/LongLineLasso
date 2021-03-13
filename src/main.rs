use clap::{App, Arg}; 
use std::io;
use lll::string_stream_editor::*;

fn process_string_stream_std_io<'a> (
    pattern: &'a str,
    before_capacity: usize,
    after_capacity: usize,
    is_ascii: bool,
    replace: Option<&str>,
    prepend: Option<&str>,
    append: Option<&str>
) {
  let stdin = io::stdin();
  let mut in_handler = stdin.lock();
  process_string_stream_bufread_bufwrite(
    &mut in_handler,
    &mut io::stdout(),
    pattern,
    before_capacity,
    after_capacity,
    is_ascii,
    replace,
    prepend,
    append
  );
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
      .about("The string to find matches for. Regexes are not supported.")
      .takes_value(true)
    )
    .arg(Arg::new("before")
      .short('b')
      .long("before")
      .value_name("INTEGER greater than or equal to 0")
      .about("Number of characters to show before the match. Defaults to 128. Set to 0 if --replace, --prepend or --append provided.")
      .takes_value(true)
    )
    .arg(Arg::new("after")
      .short('a')
      .long("after")
      .value_name("INTEGER greater than or equal to 0")
      .about("Number of characters to show after the match. Defaults to 128. Set to 0 if --replace, --prepend or --append provided.")
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
      .about("This string will be appended after all occurences of your --pattern pattern.")
      .takes_value(true)
    )
    .arg(Arg::new("prepend")
      .long("prepend")
      .value_name("STRING")
      .about("This string will be prepended before all occurences of your --pattern pattern.")
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
    process_string_stream_std_io(
      matches.value_of("pattern").expect("--pattern must be provided"),
      before,
      after,
      is_ascii,
      matches.value_of("replace"),
      matches.value_of("prepend"),
      matches.value_of("append")
    );
}

