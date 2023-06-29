//! Used to transform the expression between infix notation and prefix notation

use std::string::String;
use std::vec::Vec;

/// Used to transform the expression into prefix notation tokens in reverse order
fn infix_to_prefix_rev_tokens(expr: &str) -> Vec<String> {
    let mut stack = Vec::new();
    let mut tokens: Vec<String> = Vec::new(); // actually, it is reversed

    // preprocess, add space beside "(" and ")"
    let expr = expr.replace("(", "( ").replace(")", " )");

    for token in expr.split_whitespace().rev() {
        if token == ""{
            continue;
        }
        if token == ")" {
            stack.push(")".to_string());
        } else if token == "(" {
            while stack.last().unwrap() != ")" {
                tokens.push(stack.pop().unwrap().clone());
            }
            stack.pop();
        } else if token == "+" || token == "-" {
            // while !stack.is_empty() && stack.last().unwrap() != ")" && stack.last().unwrap() != "+" && stack.last().unwrap() != "-"{
            while !stack.is_empty()
                && (stack.last().unwrap() == "*" || stack.last().unwrap() == "/")
            {
                tokens.push(stack.pop().unwrap().clone());
            }
            stack.push(format!("{}", token));
        } else if token == "*" || token == "/" {
            // while !stack.is_empty() && stack.last().unwrap() != ")" {
            //     tokens.push(stack.pop().unwrap().clone());
            // }
            stack.push(format!("{}", token));
        } else {
            tokens.push(token.to_string());
        }
    }
    while !stack.is_empty() {
        tokens.push(stack.pop().unwrap().clone());
    }
    tokens
}

pub fn infix_to_prefix_without_bracket(expr: &str) -> String {
    let mut tokens = infix_to_prefix_rev_tokens(expr);
    tokens.reverse();
    tokens.join(" ")
}

pub fn infix_to_prefix_with_bracket(expr: &str) -> String {
    let tokens = infix_to_prefix_rev_tokens(expr);
    // pretty similar to prefix_to_infix
    let mut stack = Vec::new();

    for token in tokens{
        if token == "+" || token == "-" || token == "*" || token == "/" {
            let left = stack.pop().unwrap();
            let right = stack.pop().unwrap();
            stack.push(format!("({} {} {})", token, left, right));
        } else {
            stack.push(token.to_string());
        }
    }
    stack.pop().unwrap()
}

pub fn prefix_to_infix(expr: &str) -> String {
    let mut stack = Vec::new();

    for token in expr.split_whitespace().rev() {
        if token == "+" || token == "-" || token == "*" || token == "/" {
            let left = stack.pop().unwrap();
            let right = stack.pop().unwrap();
            stack.push(format!("({} {} {})", left, token, right));
        } else {
            stack.push(token.to_string());
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_prefix() {
        assert_eq!(infix_to_prefix_without_bracket("1 + 2"), "+ 1 2");
        assert_eq!(infix_to_prefix_without_bracket("1 + 2 * 3"), "+ 1 * 2 3");
        assert_eq!(infix_to_prefix_without_bracket("1 + 2 * 3 + 4"), "+ + 1 * 2 3 4"); //fail `"+ 1 * 2 + 3 4"`-> now pass
        assert_eq!(infix_to_prefix_without_bracket("1 + 2 * 3 + 4 * 5"), "+ + 1 * 2 3 * 4 5");
        assert_eq!(
            infix_to_prefix_without_bracket("1 + 2 * (3 + 4 * 5) "),
            "+ 1 * 2 + 3 * 4 5"
        );

        assert_eq!(infix_to_prefix_with_bracket("1 + 2"), "(+ 1 2)");
        assert_eq!(infix_to_prefix_with_bracket("1 + 2 * 3"), "(+ 1 (* 2 3))");
        assert_eq!(infix_to_prefix_with_bracket("1 + 2 * 3 + 4"), "(+ (+ 1 (* 2 3)) 4)");
        assert_eq!(
            infix_to_prefix_with_bracket("1 + 2 * 3 + 4 * 5"),
            "(+ (+ 1 (* 2 3)) (* 4 5))"
        );
        assert_eq!(
            infix_to_prefix_with_bracket("1 + 2 * (3 + 4 * 5) "),
            "(+ 1 (* 2 (+ 3 (* 4 5))))"
        );
    }

    #[test]
    fn test_prefix_to_infix() {
        assert_eq!(prefix_to_infix("+ 1 2"), "(1 + 2)");
        assert_eq!(prefix_to_infix("+ 1 * 2 3"), "(1 + (2 * 3))");
        assert_eq!(prefix_to_infix("+ + 1 * 2 3 4"), "((1 + (2 * 3)) + 4)");
    }
}
