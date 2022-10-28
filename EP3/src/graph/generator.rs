//! O módulo que reúne as funções relacionadas à geração de grafos interessantes.

use super::*;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use rayon::prelude::*;

/// Gera um grafo de ordem `v` a partir das palavras da entrada padrão e o escreve no arquivo `f`.
/// Caso a entrada seja inválida (Ex.: palavras que não tem o mesmo comprimento)
/// o programa é terminado.
///
/// Complexidade: `O(V * (V / NTHREADS) + E)`
pub fn words(words: Vec<&str>, mut f: File) {
    let v = words.len();
    let k = words[0].chars().count();

    let edges = Mutex::new(Vec::<(Handle, Handle)>::new());

    (0..v).into_par_iter().for_each(|i| {
        let mut adj = Vec::<(Handle, Handle)>::new();
        for j in i + 1..v {
            if words[i]
                .chars()
                .zip(words[j].chars())
                .map(|(c1, c2)| if c1 == c2 { 1 } else { 0 })
                .sum::<usize>()
                == k - 1
            {
                adj.push((i, j));
            }
        }
        edges.lock().unwrap().extend(adj.into_iter());
    });
    
    let edges = edges.into_inner().unwrap();
    writeln!(f, "{} {}", v, edges.len()).expect("Erro ao escrever em {path}");
    for (u, w) in edges {
        writeln!(f, "{u} {w}").expect("Erro ao escrever em {path}");
    }
}

/// Gera um grafo de ordem `v` em que a aresta entre dois vértices existe com probabilidade `p` 
/// e o escreve no arquivo `f`.
///
/// Complexidade: `O(V * (V / NTHREADS) + E)`
pub fn random(v: usize, p: f64, mut f: File) {
    let edges = Mutex::new(Vec::<(Handle, Handle)>::new());

    (0..v).into_par_iter().for_each(|i| {
        let mut adj = Vec::<(Handle, Handle)>::new();
        for j in i + 1..v {
            if rand::random::<f64>() < p {
                adj.push((i, j));
            }
        }
        edges.lock().unwrap().extend(adj.into_iter());
    });

    let edges = edges.into_inner().unwrap();
    writeln!(f, "{} {}", v, edges.len()).expect("Erro ao escrever em {path}");
    for (u, w) in edges {
        writeln!(f, "{u} {w}").expect("Erro ao escrever em {path}");
    }
}
