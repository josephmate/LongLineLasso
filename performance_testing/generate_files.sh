#!/usr/bin/env bash

if test -f "../generate_large_files/target/release/generate_large_files"; then
	#      1,000,000    1 MB
	#     10,000,000   10 MB
	#    100,000,000  100 MB
	#  1,000,000,000 1    GB
	sizes=(1000000 10000000 100000000 1000000000)
	for num_chars in "${sizes[@]}"
	do
		large_file="tmp/large_file.$num_chars.txt"
		repeated_file="tmp/repeated.$num_chars.txt"
		if test -f $large_file; then
			echo "$large_file already exists. please delete it and run again"
		else
			echo "Generating $num_chars file $large_file"
			../generate_large_files/target/release/generate_large_files --characters $num_chars --repeated_text_length 100 --times 1000 --out $large_file --repeated_text_file $repeated_file
		fi
	done
else
	echo ../generate_large_files/target/release/generate_large_files not found.
	echo Please build generate_large_files before generating the large files.
fi

