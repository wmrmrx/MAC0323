#!/usr/bin/env bash
set -e

for i in $(echo {1..3}); do
	INPUT=./tests/$i.in
	./exec < $INPUT | grep -n 'S' | cut -f1 -d: > /tmp/GOTTEN_ANSWER
	tail -n +3 $INPUT | grep -nE "$(head -n 1 $INPUT)" | cut -f1 -d: > /tmp/RIGHT_ANSWER
	if [ ! -z "$(diff /tmp/GOTTEN_ANSWER /tmp/RIGHT_ANSWER)" ]; then
		echo Erro!
		exit
	fi
done
