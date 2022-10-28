#!bin/bash
set -e

probs='
0.000050
0.000060
0.000070
0.000080
0.000090
0.000095
0.000100
0.000105
0.000110
0.000120
0.000130
0.000140
0.000150
'

v="10000"
path="/tmp/graph.in"
echo "Tamanho da componente máxima com grafo de ordem $v com"
for p in $probs; do
	out=$(./target/release/epgrafos $path random $v $p 2>/dev/null)
	echo -n "p = $p: "
	echo $(head -n 1 <<< $out)
done
