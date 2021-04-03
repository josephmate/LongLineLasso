#!/usr/bin/env bash

if [ -z "$1" ]
then
	echo "./flame_graph.sh [match|append]"
	exit -1
fi
experiment_type=$1

if [[ "match" != "$experiment_type" && "append" != "$experiment_type" ]]
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
output_path=tmp/flame_graph_$experiment_type.svg
perf_record_path=tmp/lll.$experiment_type.perf.out
folded_stacks_path=tmp/stacks.lll.$experiment_type.folded
lll_output_path=tmp/$experiment_type.lll_ascii.$num_chars.txt
if [[ "append" == "$experiment_type" ]]
then
	append_mode="--append"
	append_value="\n"
fi

kptr_status=$(cat /proc/sys/kernel/kptr_restrict)
if [[ "$kptr_status" != "0" ]]
then
	echo "WARNING: you should probably"
	echo "sudo sh -c 'echo 0 > /proc/sys/kernel/kptr_restrict'"
	echo "cat /proc/sys/kernel/kptr_restrict"
	echo "also this is helpful"
	echo "also install debug symbols TODO"
	echo "visit https://wiki.ubuntu.com/Debug%20Symbol%20Packages"
	echo "to tell you how to add debug symbol repositories to your ubuntu"
	echo "sudo apt install -y linux-image-`uname -r`-dbgsym"
	echo "will install the debug symbols"
fi



if test -f "../target/release/lll"; then
	# If running from windows linux sub system make sure to build perf from source:
	# https://stackoverflow.com/questions/60237123/is-there-any-method-to-run-perf-under-wsl
	sudo perf \
		record \
		--call-graph dwarf \
		--output $perf_record_path \
		../target/release/lll \
		  --ascii \
			--pattern $repeated_text \
			$append_mode $append_value \
			--before 100 \
			--after 100 \
		> $lll_output_path \
		< $text_file
	
	sudo chown $USER:$USER $perf_record_path

	perf script --input $perf_record_path |  inferno-collapse-perf > $folded_stacks_path
	inferno-flamegraph > $output_path < $folded_stacks_path
else
	echo ../target/release/lll not found. Please build lll before generating the flamegraph
fi

