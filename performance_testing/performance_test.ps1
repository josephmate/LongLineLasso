cd ..\
cargo build
cd performance_testing

#      1,000,000
#     10,000,000
#    100,000,000
#  1,000,000,000
#foreach ($num_chars in 1000000, 10000000, 100000000, 1000000000) {
foreach ($num_chars in 1000000) {
  Get-Location
  $text_file = "tmp\large_file.$num_chars.txt"
  $repeated_file = "tmp\repeated.$num_chars.txt"
  if (Test-Path $text_file -PathType leaf) {
    $text_file
    if (Test-Path $repeated_file -PathType leaf) {
      $repeated_file
      $repeated_text = Get-Content -Raw -Path $repeated_file
      $repeated_text

      $lllScriptBlock={
        Get-Content -Raw -Path $text_file   | ../target/debug/lll.exe --pattern $repeated_text --before 100 --after 100 > tmp\matches.lll.$num_chars.txt
      }
      # https://unix.stackexchange.com/questions/163726/limit-grep-context-to-n-characters-on-line/548716?noredirect=1#comment1183864_548716
      $rgScriptBlock={
        Get-Content -Raw -Path $text_file  | rg -r -E -o ".{0,100}$repeated_text.{0,100}" > tmp\matches.rg.$num_chars.txt
      }

      (Measure-Command  -Expression $lllScriptBlock).Milliseconds
      (Measure-Command  -Expression $rgScriptBlock).Milliseconds
    } else {
      {"$repeated_file does not exist. run .\generate_files.ps1"}
    }
  } else {
    {"$text_file does not exist. run .\generate_files.ps1"}
  }
}