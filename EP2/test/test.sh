#!/usr/bin/env bash

while true; do
	./test/gen > /tmp/tmp.in
	for st in VO ABB TR A23 ARN; do
		TMP_DIR="/tmp/text_with_st_type"
		echo $st > $TMP_DIR
		cat /tmp/tmp.in >> $TMP_DIR
		../exec < $TMP_DIR > /tmp/$st 2>/dev/null
	done
	df=$(diff /tmp/VO /tmp/ABB && diff /tmp/VO /tmp/ARN && diff /tmp/VO /tmp/A23 && diff /tmp/VO /tmp/TR)
	if [ ! -z "$df" ]; then
		echo ERRO! Resultados não são iguais!
		exit
	fi
	echo OK!
done
