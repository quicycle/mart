//! Parsing and evaluation of arp expressions with a persistent environment
use std::{collections::HashMap, convert::TryInto, fmt, fs};

use arthroprod::{
    algebra::{
        full, Alpha, ArInput, Differential, Grade, Index, Magnitude, MultiVector, Sign, Term, AR,
    },
    map,
    prelude::{Dmu, Fields, Zet_A, Zet_B, Zet_E, Zet_T, A, B, DG, E, G, T},
};
use itertools::Itertools;
use pest::{iterators::Pairs, Parser};

macro_rules! __call_ar {
    ($val:expr, $method:ident, $($param:expr),*) => {
        match $val {
            Value::Alpha(a) => Ok(Value::Alpha(a.$method($($param),*))),
            Value::Term(t) => Ok(Value::Term(t.$method($($param),*))),
            Value::MultiVector(m) => Ok(Value::MultiVector(m.$method($($param),*))),
            _ => Err(EvalError::Syntax(format!("{} is not a valid AR implementation", $val))),
        }
    };
}

macro_rules! __parser {
    ($name:ident; $res:expr) => { __parser!($name => Result<Option<Value>>; $res); };
    ($name:ident => $ret:ty; $res:expr) => {
        paste::paste! {
            fn [<parse_ $name>](pair: Pair) -> $ret {
                if let Rule::$name = pair.as_rule() {
                    $res(pair)
                } else {
                    panic!("expected {:?}, got {:?}", Rule::$name, pair.as_rule());
                }
            }
        }
    };
}

pub type Result<T> = std::result::Result<T, EvalError>;
type Pair<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum EvalError {
    #[error("{0} is invalid as an alpha value")]
    InvalidAlpha(String),

    #[error("Cannot multiply alphas by a constant")]
    InvalidAlphaProduct,

    #[error("Differentials can only be applied to multivectors")]
    InvalidDifferential,

    #[error("Invalid {0}: {} {1} {}", .2.name(), .3.name())]
    InvalidOperation(&'static str, &'static str, Value, Value),

    #[error("{0}: No such file")]
    NoSuchFile(String),

    #[error("Parse error: {0}")]
    Parse(#[from] pest::error::Error<Rule>),

    #[error("Input contained a syntax error: {0}")]
    Syntax(String),

    #[error("'{0}' is an unknown directive")]
    UnknownDirective(String),

    #[error("'{0}' is an unknown variable")]
    UnknownVariable(String),
}

/// This is the parser generated from the grammar file in this directory
///
/// It is used internally by [`Evaluator`] in order to generate the AST before beginning evaluation
/// and code execution.
#[derive(Parser)]
#[grammar = "exec/arp.pest"]
struct ARParser;

pub struct Evaluator {
    pub(super) env: HashMap<String, Value>,
    pub show_input: bool,
    pub show_comments: bool,
    pub simplify_mvecs: bool,
    // pub mvec_rep: MvecRep(Full, Shorthand, Forms),
}

impl Default for Evaluator {
    fn default() -> Self {
        let env = map! {
            "p".into() => Value::Term(term!()),
            "t".into() => Value::Term(term!(0)),
            "h".into() => Value::Term(term!(1 2 3)),
            "q".into() => Value::Term(term!(0 1 2 3)),
            "B".into() => Value::MultiVector(B()),
            "T".into() => Value::MultiVector(T()),
            "A".into() => Value::MultiVector(A()),
            "E".into() => Value::MultiVector(E()),
            "zB".into() => Value::MultiVector(Zet_B()),
            "zT".into() => Value::MultiVector(Zet_T()),
            "zA".into() => Value::MultiVector(Zet_A()),
            "zE".into() => Value::MultiVector(Zet_E()),
            "F".into() => Value::MultiVector(Fields()),
            "G".into() => Value::MultiVector(G()),
            "Dmu".into() => Value::Differential(Dmu()),
            "DG".into() => Value::Differential(DG()),
        };

        Self {
            env,
            show_input: false,
            show_comments: false,
            simplify_mvecs: false,
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluate a single line/expression
    pub fn eval(&mut self, input: &str) -> Result<()> {
        self.eval_inner(input, Rule::line)
    }

    /// Evaluate the contents of an arc file
    pub fn eval_file(&mut self, path: &str) -> Result<()> {
        self.eval_inner(
            &fs::read_to_string(path).map_err(|_| EvalError::NoSuchFile(path.into()))?,
            Rule::program,
        )
    }

    fn eval_inner(&mut self, input: &str, rule: Rule) -> Result<()> {
        let is_visible = |p: &Pair| !matches!(p.as_rule(), Rule::COMMENT | Rule::directive);

        for p in ARParser::parse(rule, input)? {
            if self.show_input && is_visible(&p) {
                println!("{}", p.as_str());
            }

            if let Some(val) = self.parse_expr(p)? {
                println!("{}\n", val);
            }
        }

        Ok(())
    }

    fn parse_expr(&mut self, p: Pair) -> Result<Option<Value>> {
        let simplify = self.simplify_mvecs;
        let mut parse_next = |exprs: &mut Pairs<Rule>| self.parse_expr(exprs.next().unwrap());

        match p.as_rule() {
            // Display comments in output if the directive is set
            Rule::COMMENT => {
                if self.show_comments {
                    print_comment(p.as_str());
                }
                Ok(None)
            }

            // Special forms

            // assignment = { ident ~ "=" ~ expr }
            Rule::assignment => {
                let mut exprs = p.into_inner();
                let ident = parse_ident(exprs.next().unwrap());
                let value = parse_next(&mut exprs)?.expect("should have had value");
                self.env.insert(ident, value.clone());
                Ok(Some(value))
            }

            // assignment = { ident ~ "=" ~ expr }
            Rule::equality => {
                let mut exprs = p.into_inner();
                let left = parse_next(&mut exprs)?.expect("should have had value");
                let right = parse_next(&mut exprs)?.expect("should have had value");
                println!("{}", left == right);
                Ok(None)
            }

            // Enable / disable directives that we know about
            Rule::directive => {
                match parse_directive(p)? {
                    (Directive::ShowInput, val) => self.show_input = val,
                    (Directive::ShowComments, val) => self.show_comments = val,
                    (Directive::Simplify, val) => self.simplify_mvecs = val,
                }
                Ok(None)
            }

            // Expressions

            // value1 value2 ... => compute the full product of values
            // NOTE: The grammar enforces that we have at least two values here
            Rule::product => {
                let mut inner = p.into_inner();
                let mut lhs = parse_next(&mut inner)?.expect("should have had lhs");

                for next in inner {
                    let rhs = self.parse_expr(next)?.expect("should have had rhs");
                    lhs = prod_values(lhs, rhs, simplify)?;
                }

                Ok(Some(lhs))
            }

            // Either a single 'division' or 'division' 'sum_op' 'division'...
            Rule::sum => self.fold_chain(p, |q| match q.into_inner().next().unwrap().as_rule() {
                Rule::sub => Box::new(sub_values),
                Rule::add => Box::new(add_values),
                _ => unreachable!(),
            }),

            // Either a single 'value' or 'value' 'div_op' 'value'...
            Rule::division => {
                self.fold_chain(p, |q| match q.into_inner().next().unwrap().as_rule() {
                    Rule::div => Box::new(div_values),
                    _ => unreachable!(),
                })
            }

            // Sub-expressions

            // Need to check for both pre and postfix operators
            Rule::value => {
                let mut inner = p.into_inner().collect_vec();

                // Helps make the match below read a little clearer
                macro_rules! parse_inner {
                    () => {
                        self.parse_expr(inner.remove(0))?
                            .expect("should have had value")
                    };
                }

                Ok(Some(match inner.len() {
                    1 => parse_inner!(),
                    2 => match inner[0].as_rule() {
                        Rule::prefix_op => parse_prefix(&mut inner)(parse_inner!())?,
                        _ => parse_postfix(&mut inner)(parse_inner!())?,
                    },
                    3 => parse_prefix(&mut inner)(parse_inner!())
                        .and_then(parse_postfix(&mut inner))?,
                    _ => unreachable!(),
                }))
            }

            // "(" 'expr' ")" so we just parse the inner expression
            Rule::group => parse_next(&mut p.into_inner()),

            // Literals
            Rule::alpha => parse_alpha(p),
            Rule::term => parse_term(p),
            Rule::magnitude => parse_magnitude(p),
            Rule::multivector => parse_multivector(p),
            Rule::differential => parse_differential(p),

            // Look up the ident in self.env and replace with the corresponding value
            Rule::ident => {
                let ident = parse_ident(p);
                self.env
                    .get(&ident)
                    .cloned()
                    .map_or_else(|| Err(EvalError::UnknownVariable(ident)), |v| Ok(Some(v)))
            }

            // EOI
            Rule::EOI => Ok(None),

            _ => Err(EvalError::Syntax(format!(
                "don't currently know how to parse {:?}",
                p
            ))),
        }
    }

    // Expect a sequence of expr OP expr OP expr... and fold from the left
    fn fold_chain(
        &mut self,
        p: Pair,
        op_func: impl Fn(Pair) -> Box<dyn Fn(Value, Value, bool) -> Result<Value>>,
    ) -> Result<Option<Value>> {
        let simplify = self.simplify_mvecs;
        let mut parse_next = |exprs: &mut Pairs<Rule>| self.parse_expr(exprs.next().unwrap());

        let mut inner = p.into_inner();
        let mut lhs = parse_next(&mut inner)?.expect("should have had lhs");

        while let Some(next) = inner.next() {
            let op = op_func(next);
            let rhs = parse_next(&mut inner)?.expect("should have had rhs");
            lhs = op(lhs, rhs, simplify)?;
        }

        Ok(Some(lhs))
    }
}

__parser!(alpha;        |p: Pair| try_alpha(p).map(|a| Some(Value::Alpha(a))));
__parser!(term;         |p: Pair| try_alpha(p).map(|a| Some(Value::Term(Term::new(None, a)))));
__parser!(multivector;  |p: Pair| Ok(Some(Value::MultiVector(MultiVector::from_ar(inner_alphas(p)?)))));
__parser!(differential; |p: Pair| Ok(Some(Value::Differential(Differential::new(&inner_alphas(p)?)))));
__parser!(magnitude;    |p: Pair| {
    let vals: Vec<usize> = p.into_inner().map(|s| s.as_str().parse().unwrap()).collect();
    Ok(Some(Value::Magnitude(vals.try_into().unwrap())))
});
__parser!(ident     => String; |p: Pair| p.as_str().into());
__parser!(directive => Result<(Directive, bool)>; |p: Pair| {
    let val = !p.as_str().starts_with("#!");
    match p.as_str().replace('!', "").as_str() {
        "#SHOW_INPUT" => Ok((Directive::ShowInput, val)),
        "#SHOW_COMMENTS" => Ok((Directive::ShowComments, val)),
        "#SIMPLIFY_MULTIVECTORS" => Ok((Directive::Simplify, val)),
        _ => Err(EvalError::UnknownDirective(p.as_str().into()))
    }
});

type ValFunc = Box<dyn FnOnce(Value) -> Result<Value>>;

fn parse_prefix(inner: &mut Vec<Pair>) -> ValFunc {
    let p = inner.remove(0).into_inner().next().unwrap();
    match p.as_rule() {
        Rule::negate => Box::new(|v| __call_ar!(v, negate,)),
        _ => unreachable!(),
    }
}

fn parse_postfix(inner: &mut Vec<Pair>) -> ValFunc {
    let p = inner.pop().unwrap().into_inner().next().unwrap();
    match p.as_rule() {
        Rule::hermitian => Box::new(|v| __call_ar!(v, hermitian,)),
        Rule::reverse => Box::new(|v| __call_ar!(v, reversed,)),
        Rule::star => Box::new(|v| __call_ar!(v, star,)),
        Rule::conjugate_grade => {
            let grades: Vec<Grade> = p
                .into_inner()
                .map(|g| g.as_str().try_into().unwrap())
                .collect_vec();
            Box::new(move |v| __call_ar!(v, conjugate_grades, grades))
        }
        Rule::conjugate_form => {
            let mut inner = p.into_inner();
            let alpha = try_alpha(inner.next().unwrap()).unwrap();
            Box::new(move |v| __call_ar!(v, conjugate_form, alpha))
        }
        _ => unreachable!("{:?}", p.as_rule()),
    }
}

fn inner_alphas(p: Pair) -> Result<Vec<Alpha>> {
    p.into_inner().map(try_alpha).collect()
}

fn try_alpha(p: Pair) -> Result<Alpha> {
    let mut inner = p.into_inner().collect_vec();
    let sign = if inner.len() == 2 {
        Sign::Neg
    } else {
        Sign::Pos
    };
    let indices = inner
        .pop()
        .unwrap()
        .as_str()
        .chars()
        .filter(|&c| !"apt".contains(c))
        .map(|c| Index::try_from_char(c).unwrap())
        .collect_vec();

    Alpha::try_from_indices(sign, &indices).map_err(EvalError::InvalidAlpha)
}

fn print_comment(s: &str) {
    if s.starts_with("/*") {
        println!("{}", s.trim_start_matches("/*").trim_end_matches("*/"));
    } else {
        println!(
            "{}",
            s.lines()
                .map(|l| l.trim_start_matches("// ").trim_start_matches("//"))
                .join("\n")
        )
    }
}

/// User supplied directives that alter execution
///
/// #DIRECTIVE enables the functionality
/// #!DIRECTIVE disables it
#[derive(Debug, Clone)]
enum Directive {
    Simplify,     // SIMPLIFY_MULTIVECTORS
    ShowInput,    // SHOW_INPUT
    ShowComments, // SHOW_COMMENTS
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Alpha(Alpha),
    Term(Term),
    Magnitude(Magnitude),
    MultiVector(MultiVector),
    Differential(Differential),
}

impl Value {
    fn name(&self) -> &str {
        match self {
            Self::Alpha(_) => "alpha",
            Self::Term(_) => "term",
            Self::Magnitude(_) => "magnitude",
            Self::MultiVector(_) => "multivector",
            Self::Differential(_) => "differential",
        }
    }
}

impl ArInput for Value {
    fn as_terms(&self) -> Vec<Term> {
        match self {
            Self::Alpha(a) => a.as_terms(),
            Self::Term(t) => t.as_terms(),
            Self::MultiVector(m) => m.as_terms(),
            _ => panic!("not a valid ArInput impl"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Alpha(v) => write!(f, "{}", v),
            Self::Term(v) => write!(f, "{}", v),
            Self::Magnitude(v) => write!(f, "{}", v),
            Self::MultiVector(v) => write!(f, "{}", v),
            Self::Differential(v) => write!(f, "{}", v),
        }
    }
}

fn sub_values(lhs: Value, rhs: Value, simplify: bool) -> Result<Value> {
    use Value::*;

    let res = match (lhs, rhs) {
        (Magnitude(m1), Magnitude(m2)) => Magnitude(m1 - m2),
        (Term(t1), Term(t2)) => MultiVector(t1 - t2),
        (Term(t), MultiVector(m)) => MultiVector(t - m),
        (MultiVector(m), Term(t)) => MultiVector(m - t),
        (MultiVector(m1), MultiVector(m2)) => MultiVector(m1 - m2),
        _ => {
            return Err(EvalError::Syntax(
                "subtraction is only valid between multivectors / terms or magnitudes".into(),
            ))
        }
    };

    match res {
        MultiVector(m) if simplify => Ok(MultiVector(m.simplified())),
        other => Ok(other),
    }
}

fn add_values(lhs: Value, rhs: Value, simplify: bool) -> Result<Value> {
    use Value::*;

    let res = match (lhs, rhs) {
        (Magnitude(m1), Magnitude(m2)) => Magnitude(m1 + m2),
        (Term(t1), Term(t2)) => MultiVector(t1 + t2),
        (Term(t), MultiVector(m)) => MultiVector(t + m),
        (MultiVector(m), Term(t)) => MultiVector(m + t),
        (MultiVector(m1), MultiVector(m2)) => MultiVector(m1 + m2),
        _ => {
            return Err(EvalError::Syntax(
                "addition is only valid between multivectors / terms or magnitudes".into(),
            ))
        }
    };

    match res {
        MultiVector(m) if simplify => Ok(MultiVector(m.simplified())),
        other => Ok(other),
    }
}

fn prod_values(lhs: Value, rhs: Value, simplify: bool) -> Result<Value> {
    use Value::*;

    macro_rules! __combine {
        (
            raw: { $($raw:tt)* };
            cases: { $($res:tt => $(($l:tt, $r:tt)),+;)+ };
        ) => {
            match (lhs, rhs) {
                $($raw)*
                $( $(($l(v1), $r(v2)) => Ok($res(v1 * v2)),)+ )+
                (v1, v2) => Ok(MultiVector(full(&v1, &v2))),
            }
        };
    }

    let res = __combine!(
        raw: {
            (Differential(d), MultiVector(m)) => Ok(Value::MultiVector(d.left_apply(&m))),
            (MultiVector(m), Differential(d)) => Ok(Value::MultiVector(d.right_apply(&m))),
            (Differential(_), _) | (_, Differential(_)) => Err(EvalError::InvalidDifferential),
            (Magnitude(_), Alpha(_)) | (Alpha(_), Magnitude(_)) => Err(EvalError::InvalidAlphaProduct),
        };
        cases: {
            Magnitude => (Magnitude, Magnitude);
            Alpha => (Alpha, Alpha);
            Term => (Magnitude, Term), (Term, Magnitude), (Term, Alpha), (Alpha, Term), (Term, Term);
            MultiVector => (Magnitude, MultiVector), (MultiVector, Magnitude);
        };
    );

    match res {
        Ok(MultiVector(m)) if simplify => Ok(MultiVector(m.simplified())),
        other => other,
    }
}

fn div_values(lhs: Value, rhs: Value, simplify: bool) -> Result<Value> {
    use Value::*;

    macro_rules! __combine {
        ($($res:tt => $(($l:tt, $r:tt)),+;)+) => {
            match (lhs, rhs) {
                $( $(($l(v1), $r(v2)) => Ok($res(v1 / v2)),)+ )+
                (v1, v2) =>  Err(EvalError::InvalidOperation("division", "/", v1, v2)),
            }
        };
    }

    let res = __combine!(
        Magnitude => (Magnitude, Magnitude);
        Alpha => (Alpha, Alpha);
        Term => (Term, Term),
                (Magnitude, Term), (Term, Magnitude),
                (Term, Alpha), (Alpha, Term);
        MultiVector => (MultiVector, MultiVector),
                       (Magnitude, MultiVector), (MultiVector, Magnitude),
                       (Alpha, MultiVector), (MultiVector, Alpha),
                       (Term, MultiVector), (MultiVector, Term);
    );

    match res {
        Ok(MultiVector(m)) if simplify => Ok(MultiVector(m.simplified())),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_alpha() {
        let res = ARParser::parse(Rule::program, "a12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().as_str(), "a12");
    }
}
