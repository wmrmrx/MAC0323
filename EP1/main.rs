mod array;
mod pentamino;
mod pilha;
mod table;

use pentamino::Pentamino;
use pilha::Pilha;
use table::{idx, ok, Table};

/// Estado do backtracking
#[derive(Debug, Default)]
struct State {
    /// Índice do pentaminó
    p_idx: usize,
    /// Índice da transformação do pentaminó em pentaminos[p_idx]
    t_idx: usize,
    /// Posição do pentaminó no tabuleiro
    pos: (i32, i32),
    /// Indica se o pentaminó foi colocado no tabuleiro
    inserted: bool,
}

/// Verifica se o tabuleiro tem sessenta espaços em vazio
fn sixty_spaces(table: &Table) -> bool {
    60 == table.iter().fold(0, |count, line| {
        count
            + line
                .iter()
                .fold(0, |count, &c| count + if c == ' ' { 1 } else { 0 })
    })
}

// Devolve o tabuleiro transposto
fn transpost(table: Table) -> Table {
    let (lin, col) = table::dim(&table);
    let mut new_table = array::new(idx(col));
    for new_lin in new_table.iter_mut() {
        *new_lin = array::new::<char>(idx(lin));
    }
    for (x, line) in table.into_iter().enumerate() {
        for (y, c) in line.into_iter().enumerate() {
            new_table[y][x] = *c;
        }
    }
    new_table
}

/// Marca a componente que contém (x, y) e devolve o tamanho da componente
fn dfs(x: i32, y: i32, marc: &mut Box<[Box<[bool]>]>, table: &Table) -> u32 {
    const DX: [i32; 4] = [1, 0, -1, 0];
    const DY: [i32; 4] = [0, 1, 0, -1];
    #[derive(Default)]
    struct DfsState {
        x: i32,
        y: i32,
        k: usize,
    }
    let mut state: Pilha<DfsState> = Pilha::new();
    state.push(DfsState { x, y, k: 0 });
    marc[idx(x)][idx(y)] = true;
    let mut count = 1;
    while !state.empty() {
        let cur = state.back();
        if cur.k == 4 {
            state.pop();
        } else {
            let (px, py) = (cur.x + DX[cur.k], cur.y + DY[cur.k]);
            cur.k += 1;
            if ok(px, py, table) && !marc[idx(px)][idx(py)] {
                marc[idx(px)][idx(py)] = true;
                state.push(DfsState { x: px, y: py, k: 0 });
                count += 1;
            }
        }
    }
    count
}

/// Verifica se todas as componentes conexas de quadrados vazios do tabuleiro são múltiplas de 5,
/// diminuindo o número de estados do backtracking que iremos passar
fn good_table(table: &Table) -> bool {
    let (lin, col) = table::dim(table);
    let mut marc = array::new::<Box<[bool]>>(idx(lin));
    for l in marc.iter_mut() {
        *l = array::new::<bool>(idx(col));
    }
    for i in 0..lin {
        for j in 0..col {
            if table[idx(i)][idx(j)] != ' ' {
                marc[idx(i)][idx(j)] = true;
            }
        }
    }
    for i in 0..lin {
        for j in 0..col {
            if !marc[idx(i)][idx(j)] {
                let cnt = dfs(i, j, &mut marc, table);
                if cnt % 5 != 0 {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let mut table: table::Table = table::scan();
    if !sixty_spaces(&table) {
        println!("-1");
        return;
    }

    // PERFORMANCE: transpor o tabuleiro para que col <= lin
    let transposed = table::dim(&table).0 < table::dim(&table).1;
    if transposed {
        table = transpost(table);
    }

    let pentaminos: [Box<[Pentamino]>; 12] = pentamino::generator::generate();
    // Marcar quais pentaminós já foram usados
    let mut marc = [false; 12];
    // Contar quantos pentaminós já foram usados
    let mut count = 0;
    let mut state: Pilha<State> = Pilha::new();
    state.push(State {
        p_idx: 0,
        t_idx: 0,
        pos: table::next_empty(0, 0, &table),
        inserted: false,
    });

    // Loop da backtracking
    while !state.empty() {
        let State {
            p_idx,
            t_idx,
            pos: (x, y),
            inserted,
        } = state.back();
        if *inserted {
            let p = &pentaminos[*p_idx][*t_idx];
            p.remove(*x, *y, &mut table);
            *inserted = false;
            marc[*p_idx] = false;
            count -= 1;
            *t_idx += 1
        } else if *p_idx >= 12 {
            state.pop();
        } else if marc[*p_idx] {
            *p_idx += 1;
        } else if *t_idx >= pentaminos[*p_idx].len() {
            *p_idx += 1;
            *t_idx = 0;
        } else {
            let p = &pentaminos[*p_idx][*t_idx];
            match p.try_put(*x, *y, &mut table) {
                Ok(()) => {
                    *inserted = true;
                    marc[*p_idx] = true;
                    count += 1;
                    if count == 12 {
                        break;
                    }
                    if good_table(&table) {
                        let next = table::next_empty(*x, *y, &table);
                        state.push(State {
                            p_idx: 0,
                            t_idx: 0,
                            pos: next,
                            inserted: false,
                        })
                    }
                }
                Err(()) => *t_idx += 1,
            };
        }
    }

    if count == 12 {
        if transposed {
            table = transpost(table);
        }
        for line in table.iter() {
            for c in line.iter() {
                print!("{c} ");
            }
            println!();
        }
    } else {
        println!("-1");
    }
}
