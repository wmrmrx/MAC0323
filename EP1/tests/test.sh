#!/usr/bin/env bash

for i in {0..11}; do
	echo Teste $i
	cat tests/$i.in
	echo Output
	./exec < tests/$i.in
done;
