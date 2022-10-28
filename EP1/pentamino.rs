//! Modelagem do pentaminó

pub mod generator;

use super::table::{idx, ok, Table};

/// Representa um pentaminó que contém os quadrados
/// (0, 0), (pt\[0\].0, pt\[0\].1), (pt\[1\].0, pt\[1\].1), ...
/// ident: Identificador do pentamino (Ex.: 'X', 'Y')
#[derive(Clone, Debug, Default)]
pub struct Pentamino {
    pub pt: [(i32, i32); 4],
    pub ident: char,
}

impl Pentamino {
    /// Checa se podemos colocar o pentaminó no tabuleiro na posição (x, y)
    fn fits_in(&self, x: i32, y: i32, table: &Table) -> bool {
        if table[idx(x)][idx(y)] != ' ' {
            return false;
        }
        for &(dx, dy) in self.pt.iter() {
            if !ok(x + dx, y + dy, table) || table[idx(x + dx)][idx(y + dy)] != ' ' {
                return false;
            }
        }
        true
    }

    /// Tenta colocar o pentaminó no tabuleiro na posição (x, y)
    pub fn try_put(&self, x: i32, y: i32, table: &mut Table) -> Result<(), ()> {
        if self.fits_in(x, y, table) {
            table[idx(x)][idx(y)] = self.ident;
            for &(dx, dy) in self.pt.iter() {
                table[idx(x + dx)][idx(y + dy)] = self.ident
            }
            Ok(())
        } else {
            Err(())
        }
    }

    /// Tira o pentaminó da posição (x, y)
    /// O pentaminó deve ter sido colocado nessa posição anteriormente
    pub fn remove(&self, x: i32, y: i32, table: &mut Table) {
        assert_eq!(table[idx(x)][idx(y)], self.ident);
        table[idx(x)][idx(y)] = ' ';
        for (dx, dy) in self.pt {
            if table[idx(x + dx)][idx(y + dy)] != self.ident {
                for line in table.iter() {
                    println!("{line:?}");
                }
            }
            assert_eq!(table[idx(x + dx)][idx(y + dy)], self.ident);
            table[idx(x + dx)][idx(y + dy)] = ' '
        }
    }
}
