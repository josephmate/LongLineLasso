#!/usr/bin/env bash

if [ -z "$1" ]
then
	echo "./flame_graph.sh [match|append]"
	exit -1
fi
experiment_type=$1

if [[ "match" -ne "$experiment_type" && "append" -ne "$experiment_type" ]]
then
	echo "./flame_graph.sh [match|append]"
	exit -1
fi
num_chars=1000000000
text_file="tmp/large_file.$num_chars.txt"
repeated_file="tmp/repeated.$num_chars.txt"
repeated_text=$(<$repeated_file)
append_mode=""
append_value=""
output_path=tmp/profile_match.svg
lll_output_path=tmp/matches.lll_ascii.$num_chars.txt
if [[ "append" -eq "$experiment_type" ]]
then
	append_mode="--append"
	append_value="\n"
	output_path=tmp/profile_append.svg
	lll_output_path=tmp/append.lll_ascii.$num_chars.txt
fi

if test -f "../target/release/lll"; then
	# If running from windows linux sub system make sure to build perf from source:
	# https://stackoverflow.com/questions/60237123/is-there-any-method-to-run-perf-under-wsl
	sudo perf \
		record \
		--call-graph dwarf \
		--output tmp/lll.perf.out \
		../target/release/lll \
		  --ascii \
			--pattern $repeated_text \
			$append_mode $append_value \
			--before 100 \
			--after 100 \
		> $lll_output_path \
		< $text_file
	
	sudo chown $USER:$USER tmp/lll.perf.out

	perf script --input tmp/lll.perf.out |  inferno-collapse-perf > tmp/stacks.folded
	inferno-flamegraph > $output_path < tmp/stacks.folded
else
	echo ../target/release/lll not found. Please build lll before generating the flamegraph
fi

