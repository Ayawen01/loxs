#[derive(Debug)]
pub enum LoxType {
    Nil,
    String(String),
    Double(f64),
    Int(i32)
}