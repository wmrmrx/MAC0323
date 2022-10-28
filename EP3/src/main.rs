//! <h1>
//!     <center>
//!         Nome: Willian Miura Mori <br \>
//!         NUSP: 12542219
//!     </center>
//! </h1>
//!
//! # Introdução
//!
//! Esse programa tem como objetivo gerar e analisar grafos interessantes e propriedades.
//!
//! Após executar o comando `make` o executável estará no diretório `target/release/epgrafos`.
//!
//! Há três modos de execução:
//!
//! ## `./epgrafos path random  v p`
//! Será gerado um grafo de ordem `v` com as arestas entre cada par de vértices existindo com probabilidade `p`. 
//! O grafo será salvo no caminho `path` e analisado logo em seguida.
//! 
//! Exemplo: `./epgrafos /tmp/graph.in random 100 0.01`
//!
//! ## `./epgrafos path words path`
//! Será gerado um grafo a partir de palavras de mesmo tamanho recebidas pela entrada padrão em que
//! duas palavras serão adjacentes se diferem em exatamente uma letra. 
//! O grafo será salvo no caminho `path` e analisado logo em seguida.
//! 
//! Exemplo: `./epgrafos /tmp/graph.in words <<< "aaa baa aba aab"`
//!
//! ## `./epgrafos path read`
//! Será lido e analisado o grafo no caminho `path` 
//!
//! Exemplo `./epgrafos /tmp/graph.in read`
//!
//! A análise é descrita em [graph::Graph::analysis]
//!
//! # Estrutura do código 
//!
//! A função [main] lida com a interação com o usuário, e as funções [random] e [words] lidam com o
//! tratamento de erros e dados (ex.: certificar-se que as palavras tem o mesmo número de
//! caracteres). A partir disso, os grafos são gerados com as funções no módulo [graph::generator].
//! O nosso grafo é definido em [graph::Graph], cujas funções importantes são [graph::Graph::from],
//! que lê o grafo a partir de um arquivo, e [graph::Graph::analysis], que irá fazer a análise de
//! nosso grafo. As tarefas básicas pedidas no EP são as funções [graph::Graph::dists] e
//! [graph::Graph::comps].
//!
//! # Detalhes de implementação
//! O grafo é indexado em 0.
//!
//! As funções [graph::Graph::dists] e [graph::Graph::comps] foram implementadas com BFS e DFS,
//! respectivamente.
//!
//! Para acelerar o programa em operações da ordem de `O(V^2)`, foram utilizadas as biblioteca externas
//! [rayon](https://github.com/rayon-rs/rayon) para utilizar paralelismo (podia ser feito apenas com a
//! biblioteca padrão, mas essa biblioteca facilita e deixa o código mais curto) e
//! [rand](https://github.com/rust-random/rand) (apenas para gerar números aleatórios)
//!
//! # Testes feitos 
//!
//! ## Componentes gigantes
//!
//! Executando `make giant` obtemos:
//!
//! ```
//! Tamanho da maior componente de um grafo aleatório de ordem 10000 com
//! p = 0.000050: 17
//! p = 0.000060: 22
//! p = 0.000070: 39
//! p = 0.000080: 78
//! p = 0.000090: 118
//! p = 0.000095: 214
//! p = 0.000100: 567
//! p = 0.000105: 1228
//! p = 0.000110: 1532
//! p = 0.000120: 3287
//! p = 0.000130: 4077
//! p = 0.000140: 5241
//! p = 0.000150: 5745
//! ```
//! Podemos ver que o tamanho da componente máxima realmente diverge ao redor de `p = 1/n`,
//! ficando pequeno quando `p = 1/n - ε` e muito grando quando `p = 1/n + ε`.
//!
//! ## Grau de separação
//!
//! Executando `make sep` obtemos:
//!
//! ```
//! Distância média entre cada par de vértices conexos com o dataset
//! desconexo.txt: NaN
//! pt3.txt: 4.17
//! pt4.txt: 5.15
//! pt5.txt: 8.81
//! pt6.txt: 1.54
//! pt7.txt: 1.24
//! sgb-words.txt: 8.34
//! ```
//!
//! Notamos que para palavras de 3,4 e 5 caracteres obtemos os resultados previstos, com um grau
//! separação pequeno (`sgb-words.txt` contém palavras em inglês com 5 caracteres). Para palavras com
//! mais de 5 palavras obtemos resultados anormais, pois a probabilidade das palavras diferirem em
//! uma letra é presumidamente menor, resultando em várias componentes desconexas, o que
//! interfere na nossa medição pois consideramos apenas pares de vértices conexos.

mod graph;

/// Função de tratamento de erros para chamar a função
/// [graph::generator::random]
fn random(path: &str, v: usize, p: f64) {
    let f = std::fs::File::create(path).expect("Erro ao criar arquivo em {path}");
    graph::generator::random(v,p,f);
}

/// Função de tratamento de erros para chamar a função
/// [graph::generator::words]
fn words(path: &str) {
    use std::io::{stdin, Read};
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Erro ao ler palavras da entrada padrão");
    let words: Vec<&str> = buffer.split_whitespace().collect();
    assert!(!words.is_empty(), "Nenhuma palavra foi lida!");
    for pair in words.windows(2) {
        assert_eq!(
            pair[0].chars().count(),
            pair[1].chars().count(),
            "Número de caracteres das palavras {} e {} não são iguais!",
            pair[0],
            pair[1]
        );
    }

    let f = std::fs::File::create(path).expect("Erro ao criar arquivo em {path}");
    graph::generator::words(words, f);
}

/// O ponto de partida do programa.
fn main() {
    let mut args = std::env::args().into_iter().skip(1);
    let path = args.next().expect("Caminho esperado!");
    match args.next().expect("Argumentos esperados!").as_str() {
        "random" => {
            let v: usize = args.next().unwrap().parse().unwrap();
            let p: f64 = args.next().unwrap().parse().unwrap();
            random(&path, v, p);
        }
        "words" => {
            words(&path);
        }
        "read" => {
        }
        s => {
            panic!("Argumento {s} não reocnhecido");
        }
    };
    graph::Graph::from(&path).analysis();
}
