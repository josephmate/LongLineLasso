#!/usr/bin/env bash
if test -f "../target/release/lll"; then
	num_chars=10000000
	text_file="tmp/large_file.$num_chars.txt"
	repeated_file="tmp/repeated.$num_chars.txt"
	repeated_text=$(<$repeated_file)
	# If running from windows linux sub system make sure to build perf from source:
	# https://stackoverflow.com/questions/60237123/is-there-any-method-to-run-perf-under-wsl
	sudo perf \
		record \
		--call-graph dwarf \
		--output tmp/lll.perf.out \
		../target/release/lll \
			--pattern $repeated_text \
			--before 100 \
			--after 100 \
		> tmp/matches.lll.$num_chars.txt \
		< $text_file
	
	sudo chown $USER:$USER tmp/lll.perf.out

	perf script --input tmp/lll.perf.out |  inferno-collapse-perf > tmp/stacks.folded
	inferno-flamegraph > tmp/profile.svg < tmp/stacks.folded
else
	echo ../target/release/lll not found. Please build lll before generating the flamegraph
fi

