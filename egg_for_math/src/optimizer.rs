//! Arithmetic Optimizer
use egg::{*, rewrite as rw};
use std::string::String;

/// Arithmetic Optimizer
struct ArithmeticOptimizer {
    /// A string that represents the current expression
    expr: String,
    /// A string that represents the optimized expression
    optimized_expr: String,
    /// A string that represents the optimizing path
    path: String,
    // TODO: use egg::Rewrite  Optimization rules 
}

impl ArithmeticOptimizer {
    /// Create a new ArithmeticOptimizer
    fn new() -> Self {
        ArithmeticOptimizer {
            expr: String::new(),
            optimized_expr: String::new(),
            path: String::new(),
        }
    }

    /// Optimize the expression
    fn optimize(&self){
        
    }
}