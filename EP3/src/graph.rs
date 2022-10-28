//! O módulo que reune as funções relacionadas a grafos.

pub mod generator;

/// O tipo que representa os vértices do nosso grafo.
/// No caso, representamos cada vértice por um inteiro positivo
/// distinto em `[0, NUMERO_DE_VERTICES)`
pub type Handle = usize;

/// A estrutura que representa um grafo.
pub struct Graph {
    /// Número de vértices do grafo
    v: usize,
    /// A lista das listas de adjacência dos vértices.
    adj: Vec<Vec<Handle>>,
}

impl Graph {
    /// Cria e devolve um grafo a partir da leitura do arquivo em `path`.
    /// Caso ocorra um erro, o programa é terminado.
    ///
    /// Complexidade: `O(V + E)`
    pub fn from(path: &str) -> Self {
        let buffer: Vec<usize> = std::fs::read_to_string(path)
            .expect("Erro ao ler {path}")
            .split_whitespace()
            .map(|s| s.parse().expect("Erro ao parsear {s}"))
            .collect();
        assert!(
            buffer.len() > 1,
            "Não há número de vértices e arestas em {path}"
        );
        let (v, e) = (buffer[0], buffer[1]);
        assert!(
            buffer.len() - 2 == 2 * e,
            "Número de arestas em {path} não corresponde ao lido"
        );

        let mut adj = vec![Vec::new(); v];
        for i in 0..e {
            let (x, y) = (buffer[2 + 2 * i], buffer[2 + 2 * i + 1]);
            assert!(x < v && y < v, "Aresta ({x}, {y}) inválida!");
            adj[x].push(y);
            adj[y].push(x);
        }
        Self {
            v, 
            adj,
        }
    }

    /// Retorna a lista `dist` das distâncias de `u` até todos os outros vértices.
    /// Caso `u` e um vértice `i` sejam desconexos, `dist[i]` será `u32::MAX = 4294967295`.
    ///
    /// Complexidade: `O(V + E)`
    pub fn dists(&self, u: Handle) -> Vec<u32> {
        let mut dists = vec![u32::MAX; self.v];
        dists[u] = 0;
        use std::collections::VecDeque;
        let mut q = VecDeque::from([u]);
        while !q.is_empty() {
            let cur: Handle = q.pop_front().unwrap();
            for p in self.adj[cur].iter() {
                let p: Handle = *p;
                if dists[p] > dists[cur] + 1 {
                    dists[p] = dists[cur] + 1;
                    q.push_back(p);
                }
            }
        }
        dists
    }

    /// Retorna uma lista contendo o tamanho das componentes conexas.
    /// Pode-se acessar o número de componentes conexas pelo tamanho da lista.
    ///
    /// Complexidade: `O(V + E)`
    pub fn comps(&self) -> Vec<u32> {
        fn dfs(cur: Handle, comp: &mut u32, vis: &mut [bool], adj: &[Vec<Handle>]) {
            vis[cur] = true;
            *comp += 1;
            for p in adj[cur].iter() {
                let p: Handle = *p;
                if !vis[p] {
                    dfs(p, comp, vis, adj);
                }
            }
        }
        let mut vis = vec![false; self.v];
        let mut comps = Vec::<u32>::new();
        for i in 0..self.v {
            if !vis[i] {
                let mut comp = 0;
                dfs(i, &mut comp, &mut vis, &self.adj);
                comps.push(comp);
            }
        }
        comps
    }

    /// Escreve para a saída padrão o resultado da análise das propriedades
    /// do grafo, que foi feita com base nos parâmetros
    /// ```
    /// Tamanho da componente máxima
    /// Tamanho médio ponderado das componentes
    /// Distância média entre cada par de vértices
    /// ```
    ///
    /// Complexidade: `O((V + E) * (V / NTHREADS))`
    pub fn analysis(&self) {
        let comps = self.comps();

        eprintln!("Tamanho da componente máxima:");
        println!("{}", comps.iter().max().unwrap());

        eprintln!("Tamanho médio ponderado das componentes");
        println!("{:.2}", comps.iter().map(|&sz| (sz as u64) * (sz as u64)).sum::<u64>() as f64 / self.v as f64);

        eprintln!("Distância média entre cada par de vértices conexos");
        use std::sync::Mutex;
        use rayon::prelude::*;
        let (sdist, ndist) = (Mutex::new(0_u64), Mutex::new(0_u64));
        (0..self.v).into_par_iter().for_each(|i| {
            let (mut s, mut n) = (0, 0);
            for d in self.dists(i).into_iter().filter(|&d| d != 0 && d != u32::MAX) {
                s += d as u64;
                n += 1;
            }
            *sdist.lock().unwrap() += s;
            *ndist.lock().unwrap() += n;
        });
        println!("{:.2}", sdist.into_inner().unwrap() as f64 / ndist.into_inner().unwrap() as f64);
    }
}
