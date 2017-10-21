pub mod matrix;

pub fn split<S: ToString, P: ToString>(record: S, cell_separator: P) -> Vec<String> {
    record
        .to_string()
        .split(&cell_separator.to_string())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

pub fn join<S1: ToString, S2: ToString>(l: Vec<S1>, sep: S2) -> String {
    l.iter().fold("".to_string(), |a, b| if a.len() > 0 {
        a + sep.to_string().as_ref()
    } else {
        a
    } + b.to_string().as_ref())
}
