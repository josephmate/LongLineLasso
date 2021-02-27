cd ..\
cargo build --release
cd performance_testing

function Run-LLL {
  param (
      $TextFile, $RepeatedText, $NumChars
  )
  Get-Content -Raw -Path $TextFile   | ../target/release/lll.exe --pattern $RepeatedText --before 100 --after 100 > tmp\matches.lll.$NumChars.txt
}
function Run-RG {
  param (
      $TextFile, $RepeatedText, $NumChars
  )
  Get-Content -Raw -Path $TextFile  | rg -r -E -o ".{0,100}$RepeatedText.{0,100}" > tmp\matches.rg.$NumChars.txt
}

#      1,000,000
#     10,000,000
#    100,000,000
#  1,000,000,000
#foreach ($num_chars in 1000000, 10000000, 100000000, 1000000000) {
foreach ($num_chars in 1000000, 10000000, 100000000) {
  Get-Location
  $text_file = "tmp\large_file.$num_chars.txt"
  $repeated_file = "tmp\repeated.$num_chars.txt"
  if (Test-Path $text_file -PathType leaf) {
    $text_file
    if (Test-Path $repeated_file -PathType leaf) {
      $repeated_file
      $repeated_text = Get-Content -Raw -Path $repeated_file
      $repeated_text

      (Measure-Command  {Run-LLL -TextFile $text_file -RepeatedText $repeated_file -NumChars $num_chars}).TotalMilliseconds
      (Measure-Command  {Run-RG -TextFile $text_file -RepeatedText $repeated_file -NumChars $num_chars}).TotalMilliseconds
    } else {
      {"$repeated_file does not exist. run .\generate_files.ps1"}
    }
  } else {
    {"$text_file does not exist. run .\generate_files.ps1"}
  }
}