cd ..\generate_large_files
cargo build
cd ..\performance_testing

#      1,000,000
#     10,000,000
#    100,000,000
#  1,000,000,000
foreach ($num_chars in 1000000, 10000000) {
  $num_chars
  ..\generate_large_files\target\debug\generate_large_files.exe --characters $num_chars --repeated_text_length 100 --times 1000 --out tmp\large_file.$num_chars.txt --repeated_text_file tmp\repeated.$num_chars.txt
}
