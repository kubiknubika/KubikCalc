// src/errors.rs

#[derive(Debug)]
pub enum CalcError {
    NotEnoughOperands,
    DivisionByZero,
    InvalidToken(String),
}