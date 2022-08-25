#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(usize),
    String(String)
}