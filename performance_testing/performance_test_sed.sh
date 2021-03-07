#!/usr/bin/env bash


function invoke_lll {
  text_file=$1
  repeated_text=$2
  num_chars=$3
  ../target/release/lll \
		--pattern $repeated_text \
		--before 100 \
		--after 100 \
		> tmp/matches.lll.$num_chars.txt < $text_file
}

function invoke_lll_ascii {
  text_file=$1
  repeated_text=$2
  num_chars=$3
  ../target/release/lll \
		--ascii \
		--pattern $repeated_text \
		--before 100 \
		--after 100 \
		> tmp/matches.lll_ascii.$num_chars.txt < $text_file
}

function invoke_sed {
  text_file=$1
  repeated_text=$2
  num_chars=$3
	sed --debug 's/'$repeated_text/$repeated_text'\n/g' $text_file \
		> tmp/append.sed.$num_chars.txt
}

function invoke_tr {
  text_file=$1
  repeated_text=$2
  num_chars=$3
  tr "0" '\n' \
		> tmp/append.tr.$num_chars.txt < $text_file
}


if test -f "../target/release/lll"; then
	#      1,000,000    1 MB
	#     10,000,000   10 MB
	#    100,000,000  100 MB
	#  1,000,000,000 1    GB
	sizes=(1000000 10000000 100000000 1000000000)
	for num_chars in "${sizes[@]}"
	do
		text_file="tmp/large_file.$num_chars.txt"
		repeated_file="tmp/repeated.$num_chars.txt"
		if test -f "$text_file"; then
			echo $text_file
			if test -f "$repeated_file"; then
				echo $repeated_file
				repeated_text=$(<$repeated_file)
				echo $repeated_text

				echo lll
				time invoke_lll "$text_file" "$repeated_text" "$num_chars"
				echo lll_ascii
				time invoke_lll_ascii "$text_file" "$repeated_text" "$num_chars"
				echo sed
				time invoke_sed "$text_file" "$repeated_text" "$num_chars"
				echo tr 
				time invoke_tr "$text_file" "$repeated_text" "$num_chars"
			else
				echo "$repeated_file does not exist. run ./generate_files.sh"
			fi
		else
			echo "$text_file does not exist. run ./generate_files.sh"
		fi
	done
else
	echo ../target/release/lll not found. Please build lll before running
	echo the performance comparison with ripgrep
fi
