#!/usr/bin/env bash
set -e

function bench {
	text=$1
	echo Teste com $text
	for st in $2 $3 $4 $5 $6; do
		TMP_DIR="/tmp/text_with_st_type"
		echo Utilizando $st...
		echo $st > $TMP_DIR
		cat test/$text.in >> $TMP_DIR
		if [ ! -d ./output/$text ]; then mkdir ./output/$text; fi
		./exec < $TMP_DIR > ./output/$text/$st
	done
	echo "-------------------------"
}

echo "-------------------------"
bench lorem_ipsum VO ABB TR A23 ARN
bench livro VO ABB TR A23 ARN
bench sorted VO ABB TR A23 ARN
bench reverse_sorted VO ABB TR A23 ARN
bench random VO ABB TR A23 ARN
