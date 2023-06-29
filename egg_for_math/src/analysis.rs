use egg::*;
use crate::expr::ArithmeticExpr;
// in this case, our analysis itself doesn't require any data, so we can just
// use a unit struct and derive Default
#[derive(Default)]  
pub struct ConstantFolding;
impl Analysis<ArithmeticExpr> for ConstantFolding {
    type Data = Option<i32>;

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        egg::merge_max(to, from)
    }

    fn make(egraph: &EGraph<ArithmeticExpr, Self>, enode: &ArithmeticExpr) -> Self::Data {
        let x = |i: &Id| egraph[*i].data;
        match enode {
            ArithmeticExpr::Num(n) => Some(*n),
            ArithmeticExpr::Add([a, b]) => Some(x(a)? + x(b)?),
            ArithmeticExpr::Mul([a, b]) => Some(x(a)? * x(b)?),
            ArithmeticExpr::Sub([a, b]) => Some(x(a)? - x(b)?),
            ArithmeticExpr::Div([a, b]) => {
                let b_data = x(b)?;
                assert_ne!(b_data, 0, "Divided by zero!");
                Some(x(a)? / b_data)
            },
            _ => None,
        }
    }

    fn modify(egraph: &mut EGraph<ArithmeticExpr, Self>, id: Id) {
        if let Some(i) = egraph[id].data {
            let added = egraph.add(ArithmeticExpr::Num(i));
            egraph.union(id, added);
        }
    }
}