use crate::{analysis::ConstantFolding, expr::ArithmeticExpr};
use egg::{rewrite as rw, *};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RULES: Vec<Rewrite<ArithmeticExpr, ConstantFolding>> = vec![
        rw!("comm-add";  "(+ ?a ?b)"        => "(+ ?b ?a)"),
        rw!("comm-mul";  "(* ?a ?b)"        => "(* ?b ?a)"),
        rw!("assoc-add"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
        rw!("assoc-mul"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),

        rw!("sub-canon"; "(- ?a ?b)" => "(+ ?a (* -1 ?b))"),
        rw!("div-canon"; "(/ ?a ?b)" => "(* ?a (/ 1 ?b))"),
        rw!("canon-sub"; "(+ ?a (* -1 ?b))"   => "(- ?a ?b)"),
        // rw!("canon-div"; "(* ?a (pow ?b -1))" => "(/ ?a ?b)" if is_not_zero("?b")),

        rw!("zero-add"; "(+ ?a 0)" => "?a"),
        rw!("zero-mul"; "(* ?a 0)" => "0"),
        rw!("one-mul";  "(* ?a 1)" => "?a"),

        rw!("add-zero"; "?a" => "(+ ?a 0)"),
        rw!("mul-one";  "?a" => "(* ?a 1)"),

        rw!("cancel-sub"; "(- ?a ?a)" => "0"),
        rw!("cancel-div"; "(/ ?a ?a)" => "1"),

        rw!("distribute"; "(* ?a (+ ?b ?c))"        => "(+ (* ?a ?b) (* ?a ?c))"),
        rw!("factor"    ; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),


        rw!("recip-mul-div"; "(* ?x (/ 1 ?x))" => "1"),


        // old rules
        // rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        // rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),

        // rw!("add-0"; "(+ ?a 0)" => "?a"),
        // rw!("mul-0"; "(* ?a 0)" => "0"),
        // rw!("mul-1"; "(* ?a 1)" => "?a"),
        // rw!("div-1"; "(/ ?a 1)" => "?a"),

        // rw!("mul-1-rev"; "?a" => "(* ?a 1)"),
        // rw!("add-sub"; "(+ ?a (* -1 ?b))" => "(- ?a ?b)"),
        // rw!("sub-add"; "(- ?a ?b)" => "(+ ?a (* -1 ?b))"),

        // rw!("mul-distributive-add"; "(* ?a (+ ?b ?c))" => "(+ (* ?a ?b) (* ?a ?c))"),
        // rw!("mul-distributive-sub"; "(* ?a (- ?b ?c))" => "(- (* ?a ?b) (* ?a ?c))"),

        // rw!("mul-distributive-add-rev"; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
        // rw!("mul-distributive-sub-rev"; "(- (* ?a ?b) (* ?a ?c))" => "(* ?a (- ?b ?c))"),

        // rw!("add-associative"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
        // rw!("add-associative-rev"; "(+ (+ ?a ?b) ?c)" => "(+ ?a (+ ?b ?c))"),

        // rw!("mul-associative"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),
        // rw!("mul-associative-rev"; "(* (* ?a ?b) ?c)" => "(* ?a (* ?b ?c))"),

        // rw!("add-sub-cancellation"; "(- (+ ?a ?b) ?b)" => "?a"),
        // rw!("mul-div-cancellation"; "(/ (* ?a ?b) ?b)" => "?a"),

    ];
}
