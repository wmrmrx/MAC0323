mod symbol_table;
use symbol_table::{SymbolTable, A23, ABB, ARN, TR, VO};

use std::time::{Duration, Instant};

fn test(mut st: impl SymbolTable<String, u64>) {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let n: usize = buffer
        .trim()
        .parse()
        .expect("Erro ao ler o número de palavras do texto");
    let mut words: Vec<String> = Vec::new();
    while words.len() < n {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        words.extend(
            buffer
                .trim()
                .split_whitespace()
                .map(|s| {
                    s.trim_matches(|c: char| c.is_ascii_punctuation())
                        .to_owned()
                })
                .filter(|s| !s.is_empty()),
        );
    }
    assert_eq!(
        words.len(),
        n,
        "Número de palavras lidas não corresponde ao dado"
    );
    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    let q: usize = buffer
        .trim()
        .parse()
        .expect("Erro ao ler o número de queries");
    let mut words = words.into_iter();
    let mut total_time = Duration::new(0, 0);
    for _ in 0..q {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim().split_whitespace().collect::<Vec<&str>>();
        assert_eq!(buffer.len(), 2, "Query inválida");
        let t: usize = buffer[0].parse().expect("Erro ao ler o tipo da query");
        match t {
            1 => {
                let x: usize = buffer[1]
                    .parse()
                    .expect("Erro ao ler o número de palavras a serem adicionadas");
                for _ in 0..x {
                    if let Some(word) = words.next() {
                        let now = Instant::now();
                        let search = st.value(&word);
                        total_time += now.elapsed();
                        if let Some(val) = search {
                            *val += 1;
                        } else {
                            let now = Instant::now();
                            st.add(word, 1);
                            total_time += now.elapsed();
                        }
                    } else {
                        break;
                    }
                }
            }
            2 => {
                let now = Instant::now();
                let res = st.value(&buffer[1].to_string());
                total_time += now.elapsed();
                println!("{}", res.map_or(0, |res| *res));
            }
            3 => {
                let now = Instant::now();
                let res = st.rank(&buffer[1].to_string());
                total_time += now.elapsed();
                println!("{}", res);
            }
            4 => {
                let k: usize = buffer[1].parse().expect("Erro ao ler o rank");
                let now = Instant::now();
                let res = st.select(k);
                total_time += now.elapsed();
                if let Some(res) = res {
                    println!("{}", *res);
                } else {
                    println!("Não há {}-ésima chave", k);
                }
            }
            _ => panic!("Operação {} não existe", t),
        }
    }
    // Saída para o STDERR
    // O tempo medido é apenas das operações da tabela de símbolos, isto é,
    // não contamos o tempo gasto com a entrada e saída ou parseamento.
    eprintln!("Tempo de execução: {:?}", total_time);
}

fn main() {
    let stdin = std::io::stdin();
    let mut st_type = String::new();
    stdin.read_line(&mut st_type).unwrap();
    match st_type.trim() {
        "VO" => test(VO::<String, u64>::new()),
        "ABB" => test(ABB::<String, u64>::new()),
        "TR" => test(TR::<String, u64>::new()),
        "ARN" => test(ARN::<String, u64>::new()),
        "A23" => test(A23::<String, u64>::new()),
        _ => panic!("Erro ao ler estrutura de dados: {}", st_type),
    }
}
