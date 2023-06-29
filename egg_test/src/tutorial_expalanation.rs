use egg::{rewrite as rw, *};

pub fn test1() {
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("div-one"; "?x" => "(/ ?x 1)"),
        rw!("unsafe-invert-division"; "(/ ?a ?b)" => "(/ 1 (/ ?b ?a))"),
        rw!("simplify-frac"; "(/ ?a (/ ?b ?c))" => "(/ (* ?a ?c) (* (/ ?b ?c) ?c))"),
        rw!("cancel-denominator"; "(* (/ ?a ?b) ?b)" => "?a"),
        rw!("times-zero"; "(* ?a 0)" => "0"),
    ];

    let start = "(/ (* (/ 2 3) (/ 3 2)) 1)".parse().unwrap();
    let end = "1".parse().unwrap();
    let mut runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&start)
        .run(rules);

    println!(
        "{}",
        runner.explain_equivalence(&start, &end).get_flat_string()
    );
}

/// This is the same as test1, but with a different start expression.
/// It's an error that shows 0=1
pub fn test2() {
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("div-one"; "?x" => "(/ ?x 1)"),
        rw!("unsafe-invert-division"; "(/ ?a ?b)" => "(/ 1 (/ ?b ?a))"),
        rw!("simplify-frac"; "(/ ?a (/ ?b ?c))" => "(/ (* ?a ?c) (* (/ ?b ?c) ?c))"),
        rw!("cancel-denominator"; "(* (/ ?a ?b) ?b)" => "?a"),
        rw!("times-zero"; "(* ?a 0)" => "0"),
    ];

    let start = "0".parse().unwrap();
    let end = "1".parse().unwrap();
    let mut runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&start)
        .run(rules);

    println!(
        "{}",
        runner.explain_equivalence(&start, &end).get_flat_string()
    );
}

/// TreeExplanation
pub fn test3(){
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("div-one"; "?x" => "(/ ?x 1)"),
        rw!("unsafe-invert-division"; "(/ ?a ?b)" => "(/ 1 (/ ?b ?a))"),
        rw!("simplify-frac"; "(/ ?a (/ ?b ?c))" => "(/ (* ?a ?c) (* (/ ?b ?c) ?c))"),
        rw!("cancel-denominator"; "(* (/ ?a ?b) ?b)" => "?a"),
        rw!("times-zero"; "(* ?a 0)" => "0"),

        // 补充
        // rw!("add-zero"; "(+ ?a 0)" => "?a"), // 不需要, constant_fold 会做
        // rw!("constant_fold"; "(+ ?a ?b)" => { format!("{}", a.as_int()? + b.as_int()?) }),
        rw!("constant_fold"; "?n@Num(?c)" => "?c"),
    ];

    let start = "(+ 1 (- a (* (- 2 1) a)))".parse().unwrap();
    let end = "1".parse().unwrap();
    let mut runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&start)
        .run(rules);

    println!(
        "{}",
        runner.explain_equivalence(&start, &end).get_string()
    );
}


pub fn test4(){
    define_language! {
        enum SimpleMath {
            "+" = Add([Id; 2]),
            "*" = Mul([Id; 2]),
            Num(i32),
            Symbol(Symbol),
        }
    }

    // in this case, our analysis itself doesn't require any data, so we can just
    // use a unit struct and derive Default
    #[derive(Default)]  
    struct ConstantFolding;
    impl Analysis<SimpleMath> for ConstantFolding {
        type Data = Option<i32>;

        fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
            egg::merge_max(to, from)
        }

        fn make(egraph: &EGraph<SimpleMath, Self>, enode: &SimpleMath) -> Self::Data {
            let x = |i: &Id| egraph[*i].data;
            match enode {
                SimpleMath::Num(n) => Some(*n),
                SimpleMath::Add([a, b]) => Some(x(a)? + x(b)?),
                SimpleMath::Mul([a, b]) => Some(x(a)? * x(b)?),
                _ => None,
            }
        }

        fn modify(egraph: &mut EGraph<SimpleMath, Self>, id: Id) {
            if let Some(i) = egraph[id].data {
                let added = egraph.add(SimpleMath::Num(i));
                egraph.union(id, added);
            }
        }
    }

    let rules = &[
        rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),

        rw!("add-0"; "(+ ?a 0)" => "?a"),
        rw!("mul-0"; "(* ?a 0)" => "0"),
        rw!("mul-1"; "(* ?a 1)" => "?a"),
    ];

    let expr = "(+ 0 (* (+ 9 -3) foo))".parse().unwrap();
    let runner = Runner::<SimpleMath, ConstantFolding, ()>::default().with_expr(&expr).run(rules);
    
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    println!("best cost: {}-{}", best_cost, best_expr);
}