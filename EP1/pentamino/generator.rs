//! Geração dos pentaminós possíveis para serem colocados no backtracking

use super::Pentamino;
use crate::array;
use crate::pilha::Pilha;

/// Simetrias de um pentaminó
enum Symmetry {
    None,
    Reflexive,
    Radial,
}

/// Rotaciona o pentaminó no sentido anti-horário
fn rotate(p: &mut Pentamino) {
    for pt in p.pt.iter_mut() {
        (pt.0, pt.1) = (pt.1, -pt.0);
    }
}

/// Reflete o pentaminó em torno do eixo y
fn reflect(p: &mut Pentamino) {
    for pt in p.pt.iter_mut() {
        pt.1 = -pt.1;
    }
}

/// Transforma o pentaminó de modo que as x >= 0 e se x == 0, y >= 0 para garantir unicidade dos
/// pentaminós.
fn normalized(p: &Pentamino) -> Pentamino {
    let mut p: Pentamino = p.clone();
    let (mut mx, mut my) = (0, i32::MAX);
    for (x, _) in p.pt.iter() {
        mx = mx.min(*x);
    }
    if mx == 0 {
        my = my.min(0);
    }
    for (x, y) in p.pt.iter() {
        if *x == mx {
            my = my.min(*y);
        }
    }
    for (x, y) in p.pt.iter_mut() {
        *x -= mx;
        *y -= my;
        if *x == 0 && *y == 0 {
            *x = -mx;
            *y = -my;
        }
    }
    p
}

/// Insere as rotações do pentaminó p na pilha
fn push_rotations(p: &mut Pentamino, pilha: &mut Pilha<Pentamino>) {
    for _ in 0..4 {
        pilha.push(normalized(p));
        rotate(p);
    }
}

/// Gera as transformações do pentaminó
fn transformations(mut p: Pentamino, s: Symmetry) -> Box<[Pentamino]> {
    let mut pilha = Pilha::new();
    match s {
        Symmetry::Radial => pilha.push(normalized(&p)),
        Symmetry::Reflexive => push_rotations(&mut p, &mut pilha),
        Symmetry::None => {
            push_rotations(&mut p, &mut pilha);
            reflect(&mut p);
            push_rotations(&mut p, &mut pilha);
        }
    }
    let mut transformations = array::new(pilha.size());
    for (idx, t) in pilha.iter().enumerate() {
        transformations[idx] = t.clone();
    }
    transformations
}

/// Gera os 12 pentaminós e respectivas possíveis transformações
pub fn generate() -> [Box<[Pentamino]>; 12] {
    const I: Pentamino = Pentamino {
        pt: [(0, 1), (0, 2), (0, -1), (0, -2)],
        ident: 'I',
    };
    const F: Pentamino = Pentamino {
        pt: [(0, -1), (1, 1), (0, 1), (-1, 0)],
        ident: 'F',
    };
    const N: Pentamino = Pentamino {
        pt: [(0, -1), (1, 0), (1, 1), (1, 2)],
        ident: 'N',
    };
    const P: Pentamino = Pentamino {
        pt: [(0, 1), (0, 2), (1, 1), (1, 2)],
        ident: 'P',
    };
    const Y: Pentamino = Pentamino {
        pt: [(1, 0), (0, 1), (0, -1), (0, -2)],
        ident: 'Y',
    };
    const W: Pentamino = Pentamino {
        pt: [(0, -1), (1, 0), (1, 1), (-1, -1)],
        ident: 'W',
    };
    const Z: Pentamino = Pentamino {
        pt: [(0, 1), (0, -1), (1, 1), (-1, -1)],
        ident: 'Z',
    };
    const V: Pentamino = Pentamino {
        pt: [(1, 0), (-1, 0), (1, 1), (1, 2)],
        ident: 'V',
    };
    const L: Pentamino = Pentamino {
        pt: [(0, 1), (0, 2), (0, 3), (1, 0)],
        ident: 'L',
    };
    const U: Pentamino = Pentamino {
        pt: [(1, 0), (-1, 0), (1, 1), (-1, 1)],
        ident: 'U',
    };
    const T: Pentamino = Pentamino {
        pt: [(0, 1), (0, 2), (-1, 2), (1, 2)],
        ident: 'T',
    };
    const X: Pentamino = Pentamino {
        pt: [(1, 0), (-1, 0), (0, 1), (0, -1)],
        ident: 'X',
    };
    [
        transformations(F, Symmetry::None),
        transformations(N, Symmetry::None),
        transformations(P, Symmetry::None),
        transformations(Y, Symmetry::None),
        transformations(L, Symmetry::None),
        transformations(W, Symmetry::Reflexive),
        transformations(I, Symmetry::Reflexive),
        transformations(Z, Symmetry::Reflexive),
        transformations(V, Symmetry::Reflexive),
        transformations(U, Symmetry::Reflexive),
        transformations(T, Symmetry::Reflexive),
        transformations(X, Symmetry::Radial),
    ]
}
