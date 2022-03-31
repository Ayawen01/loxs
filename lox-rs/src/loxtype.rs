#[derive(Debug)]
pub enum LoxType {
    Id(String),
    String(String),
    Double(f64),
    Nil,
}