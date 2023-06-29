mod optimizer;
mod expr;
mod analysis;
mod rules;

use egg::*;
use rules::RULES;
// use trans_expr::{infix_to_prefix_with_bracket, prefix_to_infix}; // more readable

pub fn optimize_expr(expression: &str) -> String{
    let rules = RULES.as_slice();
    let start = expression.parse().unwrap();
    let runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&start)
        .run(rules);
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    // print the best expression and its cost
    println!("best: {} = {}", best_cost, best_expr);
    format!("{}", best_expr)
}

#[cfg(test)]
mod tests {
    use crate::optimize_expr;

    #[test]
    fn it_works() {
        // assert_eq!(optimize_expr("(/ 1 0)"), "1");
        // assert_eq!(optimize_expr("(/ 1 2)"), "(/ 1 2)");
        assert_eq!(optimize_expr("(/ x 1)"), "x");
        assert_eq!(optimize_expr("(+ a (- b a))"), "b"); 
        assert_eq!(optimize_expr("(+ 1 (- a (* (+ b 1) a)))"), "(- 1 (* a b))");
        assert_eq!(optimize_expr("(* (* (* x 1) 1) 1)"), "x");
        // todo: 问题有:
        // 1. assert_eq!(optimize_expr("(/ 1 2)"), "(/ 1 2)"); 无法通过, Analysis::make 不应该计算 1/2 的值. 会报错thread 'tests::it_works' panicked at 'attempt to multiply with overflow'
        // 2. 非常慢(添加add-sub和sub-add后)
    }
}
