use crate::digraph::Digraph;
use std::cmp::PartialEq;
use std::collections::HashSet;

/// Para representar conjuntos e intervalos
#[derive(Debug, PartialEq)]
enum Matcher {
    Set(Box<[char]>),
    Interval(char, char),
}

/// Átomo da nossa regexp
#[derive(Debug, PartialEq)]
enum Ident {
    Asterisk,
    Plus,
    Bar,
    ParenthesisOpen,
    ParenthesisClose,
    Point,
    // Alocar na heap para diminuir a memória do enum
    Match(Box<(Matcher, bool)>),
    Char(char),
}

/// IMPLEMENTAÇÃO:
/// Ao invés de checarmos com ifs manualmente se um caractere dá match
/// com um ponto, intervalo ou ele mesmo, implementamos o operator
/// Ident == char para fazer isso.
impl PartialEq<char> for Ident {
    fn eq(&self, rhs: &char) -> bool {
        if let Ident::Char(c) = self {
            c == rhs
        } else if let Ident::Match(ptr) = self {
            let (matcher, negation) = &**ptr;
            use Matcher::*;
            negation
                ^ match matcher {
                    Set(set) => set.iter().any(|ch| ch == rhs),
                    Interval(a, b) => (a..=b).contains(&rhs),
                }
        } else {
            *self == Ident::Point
        }
    }
}

#[derive(Debug)]
pub struct Nfa {
    re: Box<[Ident]>,
    g: Digraph,
    m: usize,
}

impl Nfa {
    fn process_regexp(regexp: &str) -> Box<[Ident]> {
        let mut res = Vec::<Ident>::new();

        let mut escape = false;

        let mut set = Vec::<char>::new();
        let (mut in_bracket, mut negation, mut interval) = (false, false, false);

        use Ident::*;
        for (i, c) in regexp.chars().enumerate() {
            if in_bracket {
                if escape {
                    set.push(c);
                    escape = false;
                } else {
                    match c {
                        ' ' => {}
                        '*' | '+' | '.' | '|' | '[' | '(' | ')' => {
                            panic!("Caractere \"{c}\" inválido em {i}")
                        }
                        ']' => {
                            if interval {
                                assert_eq!(set.len(), 2, "Intervalo inválido em {i}");
                                res.push(Match(Box::new((
                                    Matcher::Interval(set[0], set[1]),
                                    negation,
                                ))));
                                set.clear();
                                interval = false;
                            } else {
                                res.push(Match(Box::new((
                                    Matcher::Set(std::mem::take(&mut set).into_boxed_slice()),
                                    negation,
                                ))));
                            }
                            in_bracket = false;
                            negation = false;
                        }
                        '-' => {
                            assert!(!interval, "Caractere \"{c}\" inválido em {i}");
                            assert_eq!(set.len(), 1, "Intervalo inválido em {i}");
                            interval = true;
                        }
                        '^' => {
                            assert!(
                                set.is_empty() && !negation && !interval,
                                "Caractere \"{c}\" inválido em {i}"
                            );
                            negation = true;
                        }
                        '\\' => escape = true,
                        c => set.push(c),
                    }
                }
            } else if escape {
                res.push(Char(c));
                escape = false;
            } else {
                match c {
                    ' ' => {}
                    '*' => res.push(Asterisk),
                    '+' => res.push(Plus),
                    '.' => res.push(Point),
                    '|' => res.push(Bar),
                    '[' => in_bracket = true,
                    ']' => panic!("Bracket inválido em {i}!"),
                    '(' => res.push(ParenthesisOpen),
                    ')' => res.push(ParenthesisClose),
                    '-' => panic!("Hífen inválido em {i}!"),
                    '^' => panic!("Circumflexo inválido em {i}!"),
                    '\\' => escape = true,
                    c => res.push(Char(c)),
                }
            }
        }
        if escape {
            panic!("Escape inesperado no final da expressão irregular!");
        }
        if in_bracket {
            panic!("Bracket não fechado no final da expressão irregular!");
        }
        res.into_boxed_slice()
    }

    pub fn new(regexp: &str) -> Self {
        let mut ops = Vec::<usize>::new();
        let re: Box<[Ident]> = Self::process_regexp(regexp);
        let m = re.len();
        let mut g = Digraph::new(m + 1);
        for i in 0..m {
            use Ident::*;
            let mut lp = i;
            if let ParenthesisOpen | Bar = re[i] {
                ops.push(i);
            } else if re[i] == ParenthesisClose {
                let mut ors = Vec::<usize>::new();
                let mut or = ops.pop().expect("Erro ao processar a expressão regular!");
                while re[or] == Bar {
                    ors.push(or);
                    g.add(or, i);
                    or = ops.pop().expect("Erro ao processar a expressão regular!");
                }
                lp = or;
                for or in ors {
                    g.add(lp, or + 1);
                    assert!(lp < or + 1);
                }
            }
            if i + 1 < m {
                if let Asterisk | Plus = re[i + 1] {
                    if re[i + 1] == Asterisk {
                        g.add(lp, i + 1);
                    }
                    g.add(i + 1, lp);
                }
            }
            if let ParenthesisOpen | ParenthesisClose | Asterisk | Plus = re[i] {
                g.add(i, i + 1);
            }
        }
        assert!(ops.is_empty(), "Erro ao processar a expressão regular!");
        Self { re, g, m }
    }

    pub fn recognizes(&self, txt: &str) -> bool {
        let mut pc = HashSet::<usize>::new();
        let mut vis = vec![false; self.g.size()];
        for &v in self.g.unvisited_from(0, &mut vis).iter() {
            pc.insert(v);
        }
        for c in txt.chars() {
            let mut mtch = HashSet::<usize>::new();
            for &v in pc.iter() {
                if v < self.m && self.re[v] == c {
                    mtch.insert(v + 1);
                }
            }
            self.g.reset_visited(&mut vis, pc.iter());
            pc.clear();
            for v in mtch {
                for &v in self.g.unvisited_from(v, &mut vis).iter() {
                    pc.insert(v);
                }
            }
        }
        pc.iter().any(|&v| v == self.m)
    }
}
