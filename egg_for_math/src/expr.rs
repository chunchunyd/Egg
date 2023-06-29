use egg::*;
// use std::string::String;

define_language! {
    /// Arithmetic Expression
    pub enum ArithmeticExpr {
        Num(i32),
        Symbol(Symbol),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        "-" = Sub([Id; 2]),
        "/" = Div([Id; 2]),
    }
}