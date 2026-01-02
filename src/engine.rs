// src/engine.rs

use crate::errors::CalcError;
use std::f64::consts::{PI, E};

pub struct KubikEngine {
    stack: Vec<f64>,
}

impl KubikEngine {
    pub fn new() -> Self {
        KubikEngine { stack: Vec::new() }
    }

    pub fn get_stack(&self) -> &Vec<f64> {
        &self.stack
    }

    pub fn eval(&mut self, token: &str) -> Result<(), CalcError> {
        if let Ok(num) = token.parse::<f64>() {
            self.stack.push(num);
            return Ok(());
        }

        match token {
            "pi" => self.stack.push(PI),
            "e" => self.stack.push(E),
            "clear" => self.stack.clear(),
            "drop" => { self.stack.pop(); },
            _ => self.process_op(token)?,
        }
        Ok(())
    }

    // Вспомогательная функция для факториала (5! = 120)
    // Работает только с положительными целыми числами
    fn calculate_factorial(n: f64) -> Result<f64, CalcError> {
        if n < 0.0 || n.fract() != 0.0 {
            return Err(CalcError::InvalidToken("Factorial requires positive integer".to_string()));
        }
        if n > 170.0 { // 171! уже переполняет f64
             return Ok(f64::INFINITY);
        }
        let mut result = 1.0;
        for i in 1..=(n as u64) {
            result *= i as f64;
        }
        Ok(result)
    }

    fn process_op(&mut self, op: &str) -> Result<(), CalcError> {
        // --- Унарные операции (требуют 1 число) ---
        let unary_ops = [
            "sqrt", "sin", "cos", "tan", "asin", "acos", "atan",
            "sinh", "cosh", "tanh", "ln", "log", "deg", "rad", 
            "abs", "round", "ceil", "floor", "recip", "!"
        ];

        if unary_ops.contains(&op) {
            if self.stack.is_empty() { return Err(CalcError::NotEnoughOperands); }
            let a = self.stack.pop().unwrap();
            
            let res = match op {
                "sqrt" => if a < 0.0 { return Err(CalcError::InvalidToken("Sqrt of negative".to_string())); } else { a.sqrt() },
                "sin" => a.sin(),
                "cos" => a.cos(),
                "tan" => a.tan(),
                "asin" => a.asin(),
                "acos" => a.acos(),
                "atan" => a.atan(),
                "sinh" => a.sinh(),
                "cosh" => a.cosh(),
                "tanh" => a.tanh(),
                "ln" => a.ln(),       // Натуральный логарифм (base e)
                "log" => a.log10(),   // Десятичный логарифм
                "deg" => a.to_degrees(),
                "rad" => a.to_radians(),
                "abs" => a.abs(),
                "round" => a.round(),
                "ceil" => a.ceil(),
                "floor" => a.floor(),
                "recip" => a.recip(), // 1/x
                "!" => Self::calculate_factorial(a)?, // Вызываем наш метод
                _ => unreachable!(),
            };
            self.stack.push(res);
            return Ok(());
        }

        // --- Бинарные операции (требуют 2 числа) ---
        let binary_ops = ["+", "-", "*", "/", "^", "pow", "root", "log_base"]; // root - корень N степени
        
        if binary_ops.contains(&op) {
            if self.stack.len() < 2 { return Err(CalcError::NotEnoughOperands); }
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();

            let res = match op {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => if b == 0.0 { 
                    self.stack.push(a); self.stack.push(b); 
                    return Err(CalcError::DivisionByZero); 
                } else { a / b },
                "^" | "pow" => a.powf(b),
                "root" => a.powf(b.recip()), // Корень степени B из A
                "log_base" => a.log(b),      // Логарифм A по основанию B
                _ => unreachable!(),
            };
            self.stack.push(res);
            return Ok(());
        }

        Err(CalcError::InvalidToken(op.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx(a: f64, b: f64) {
        if (a - b).abs() > 1e-10 { panic!("{} != {}", a, b); }
    }

    #[test]
    fn test_factorial() {
        let mut eng = KubikEngine::new();
        eng.eval("5").unwrap();
        eng.eval("!").unwrap(); // 5! = 120
        assert_approx(eng.get_stack()[0], 120.0);
    }

    #[test]
    fn test_trig_inverse() {
        let mut eng = KubikEngine::new();
        eng.eval("1").unwrap();
        eng.eval("atan").unwrap(); // atan(1) = pi/4
        eng.eval("4").unwrap();
        eng.eval("*").unwrap();
        assert_approx(eng.get_stack()[0], PI);
    }
}