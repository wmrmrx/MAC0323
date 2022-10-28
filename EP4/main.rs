mod digraph;
mod nfa;

fn main() {
    let stdin = std::io::stdin();
    let mut regexp = String::new();
    stdin.read_line(&mut regexp).unwrap();
    let automata = nfa::Nfa::new(regexp.trim());
    let mut n = String::new();
    stdin.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    for _ in 0..n {
        let mut txt = String::new();
        stdin.read_line(&mut txt).unwrap();
        if automata.recognizes(txt.trim()) {
            println!("S");
        } else {
            println!("N");
        }
    }
}
