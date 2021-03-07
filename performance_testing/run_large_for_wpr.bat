@echo off
setlocal enabledelayedexpansion
IF EXIST "..\target\release\lll.exe" (
  set "num_chars=10000000"
  echo !num_chars!
  set "text_file=tmp\large_file.!num_chars!.txt"
  echo !text_file!
  set "repeated_file=tmp\repeated.!num_chars!.txt"
  echo !repeated_file!
  set "output_matches=tmp\matches.lll.!num_chars!.txt"
  echo !output_matches!

  for /f "delims=" %%x in (!repeated_file!) do set repeated_text=%%x
  echo !repeated_text!

  echo "../target/release/lll.exe --pattern !epeated_text! --before 100 --after 100 > !output_matches!< !text_file!"
  ..\target\release\lll.exe --pattern !repeated_text! --before 100 --after 100 > !output_matches! < !text_file!
) ELSE (
  echo "../target/release/lll.exe does not exist. Please run `cargo build --release`"
)



