cd ..\generate_large_files
cargo build --release
cd ..\performance_testing

# 10^3 instead of Base 2^10
#      1,000,000B = 1,000KB =     1 MB
#     10,000,000B           =    10 MB
#    100,000,000B           =   100 MB
#  1,000,000,000B           = 1,000 MB
foreach ($num_chars in 1000000, 10000000, 100000000, 1000000000) {
  $large_file="tmp\large_file.$num_chars.txt"
  $repeated_file="tmp\repeated.$num_chars.txt"
  if (Test-Path $large_file -PathType leaf) {
    Write-Output "$large_file already exists. please delete it and run again"
  } else {
    Write-Output "Generating $num_chars file $large_file"
    ..\generate_large_files\target\release\generate_large_files.exe --characters $num_chars --repeated_text_length 100 --times 1000 --out $large_file --repeated_text_file $repeated_file
  }
}
