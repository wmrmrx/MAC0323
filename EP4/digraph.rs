#[derive(Debug)]
pub struct Digraph {
    adj: Box<[Vec<usize>]>,
}

impl Digraph {
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n].into_boxed_slice(),
        }
    }

    pub fn size(&self) -> usize {
        self.adj.len()
    }

    pub fn add(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }

    pub fn unvisited_from(&self, v: usize, vis: &mut [bool]) -> Box<[usize]> {
        if vis[v] {
            return Box::new([]);
        }
        let mut visited = Vec::<usize>::new();

        fn dfs(cur: usize, vis: &mut [bool], adj: &[Vec<usize>], visited: &mut Vec<usize>) {
            vis[cur] = true;
            visited.push(cur);
            for prox in adj[cur].iter() {
                if !vis[*prox] {
                    dfs(*prox, vis, adj, visited);
                }
            }
        }

        dfs(v, vis, &self.adj, &mut visited);
        for v in visited.iter() {
            vis[*v] = false;
        }
        visited.into_boxed_slice()
    }

    pub fn reset_visited<'a>(&self, vis: &mut [bool], visited: impl Iterator<Item = &'a usize>) {
        for &v in visited {
            vis[v] = false;
        }
    }
}
