use std::fmt;

#[derive(Debug)]
pub enum CalculatorError {
    DivisionByZero,
    InvalidInput,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Помилка: ділення на нуль."),
            CalculatorError::InvalidInput => write!(f, "Помилка: некоректний ввід."),
        }
    }
}

pub struct Calculator {
    pub memory: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { memory: 0.0 }
    }

    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.memory = result;
        result
    }

    pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
        let result = a - b;
        self.memory = result;
        result
    }

    pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let result = a * b;
        self.memory = result;
        result
    }

    pub fn divide(&mut self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            let result = a / b;
            self.memory = result;
            Ok(result)
        }
    }
}
