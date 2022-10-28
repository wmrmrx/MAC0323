#!bin/bash
set -e

path="/tmp/graph.in"
pref="./test/data"
data=$(ls $pref)

echo Distância média entre cada par de vértices conexos com dataset:
for dataset in $data; do
	out=$(./target/release/epgrafos $path words < $pref/$dataset  2>/dev/null)
	echo -n "$dataset: "
	echo $(tail -n 1 <<< $out)
done
