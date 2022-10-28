//! Utilidades para ler e lidar com o tabuleiro

use super::array;
use std::convert::TryInto;

/// '~': Quadrado não pode ser ocupado
/// ' ': Quadrado vazio
/// 'X': Quadrado ocupado pelo pentamino X
/// ...
pub type Table = Box<[Box<[char]>]>;

/// Função auxiliar para indexação do tabuleiro
pub fn idx<T: TryInto<usize>>(i: T) -> usize
where
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    i.try_into().unwrap()
}

/// Checa se (x, y) é uma posição válida no tabuleiro
pub fn ok(x: i32, y: i32, table: &Table) -> bool {
    let dim = dim(table);
    0 <= x && x < dim.0 && 0 <= y && y < dim.1
}

/// Devolve as dimensões do tabuleiro
pub fn dim(table: &Table) -> (i32, i32) {
    (
        table.len().try_into().unwrap(),
        table[0].len().try_into().unwrap(),
    )
}

/// Devolve a próxima posição vazia no tabuleiro a partir de (x, y)
pub fn next_empty(mut x: i32, mut y: i32, table: &Table) -> (i32, i32) {
    while table[idx(x)][idx(y)] != ' ' {
        if y + 1 >= dim(table).1 {
            x += 1;
            y = 0;
        } else {
            y += 1;
        }
    }
    (x, y)
}

/// Lê o tabuleiro
pub fn scan() -> Table {
    let mut s = String::new();
    use std::io::Read;
    std::io::stdin()
        .read_to_string(&mut s)
        .unwrap_or_else(|e| panic!("Erro de leitura: {:?}", e));
    let lines = s.lines().count();
    assert!(lines > 0, "Input vazio!");
    let mut table: Table = array::new(lines);
    let mut columns: Option<usize> = None;
    for (num, line) in s.lines().enumerate() {
        table[num] = line
            .trim()
            .split_whitespace()
            .map(|b| match b {
                "0" => ' ',
                "1" => '~',
                _ => panic!("Erro ao ler o elemento: {} na linha {}", b, num),
            })
            .collect();
        if let Some(col) = columns {
            if col != table[num].len() {
                panic!(
                    "Linhas {} e {} possuem um número de colunas diferentes!",
                    num - 1,
                    num
                );
            }
        }
        assert!(table[num].len() > 0, "Linha {} vazia!", num);
        columns = Some(table[num].len());
    }
    table
}
