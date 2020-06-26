use std::collections::HashMap;

extern crate pest;
extern crate pest_derive;

use pest::Parser;

use crate::lib_core::{
    math::{FixedNumber, Range, Vec3d},
    Aabb,
};

#[derive(Parser)]
#[grammar = "scripting/slisp.pest"]
struct SlispParser;

macro_rules! check {
    ($cond:expr, ($($arg:tt)*)) => {
        if $cond {
            return Err(format!($($arg)*));
        }
    };
}

macro_rules! str_err {
    ($($arg:tt)*) => {
        Err(format!($($arg)*));
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Empty,
    Bool(bool),
    Number(FixedNumber),
    Str(String),
    Symbol(String),
    Func(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Sexpr(Vec<Expr>),
    Qexpr(Vec<Expr>),
}

fn default_env() -> HashMap<String, Expr> {
    let mut env = HashMap::<String, Expr>::new();
    return env;
}

pub struct Slisp {
    environment: HashMap<String, Expr>,
}

impl Slisp {
    pub fn new() -> Self {
        Self {
            environment: default_env(),
        }
    }

    pub fn read_str(&self, slisp: String) -> Result<Expr, String> {
        let pairs = SlispParser::parse(Rule::slisp, &slisp);

        let mut exprs: Vec<Expr> = vec![];

        match pairs {
            Ok(pairs) => {
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::slisp => {
                            for pair in pair.into_inner() {
                                let val = read(pair)?;

                                exprs.push(val);
                            }
                        }
                        _ => unimplemented!("Not implemented for Rule '{}'!", pair),
                    }
                }
            }
            Err(error) => {
                return str_err!("PARSING ERR: {}", error);
            }
        }

        // Join up all exprs into one for evaluating
        Ok(Expr::Sexpr(exprs))
    }

    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Atom(a) => {
                return match a {
                    Atom::Bool(b) => format!("{}", b),
                    Atom::Empty => format!("()"),
                    Atom::Number(n) => format!("{:?}", (Into::<f32>::into(*n))),
                    Atom::Str(s) => s.to_string(),
                    Atom::Func(s) => s.to_string(),
                    Atom::Symbol(s) => s.to_string(),
                };
            }
            Expr::Qexpr(exprs) => {
                let mut s = String::from("{");
                for (i, expr) in exprs.iter().enumerate() {
                    if i == 0 {
                        s = format!("{}{}", s, self.print(expr));
                    } else {
                        s = format!("{} {}", s, self.print(expr));
                    }
                }

                s = format!("{}}}", s);

                return s;
            }
            Expr::Sexpr(exprs) => {
                let mut s = String::from("(");
                for (i, expr) in exprs.iter().enumerate() {
                    if i == 0 {
                        s = format!("{}{}", s, self.print(expr));
                    } else {
                        s = format!("{} {}", s, self.print(expr));
                    }
                }

                s = format!("{})", s);

                return s;
            }
        };
    }

    pub fn eval(&mut self, expr: Expr) -> Result<Expr, String> {
        return eval(expr, &mut self.environment);
    }
}

fn read(pair: pest::iterators::Pair<Rule>) -> Result<Expr, String> {
    match pair.as_rule() {
        Rule::expr => {
            for pair in pair.into_inner() {
                return read(pair);
            }

            // Really, this should never be reached
            return str_err!("PARSING: Reached an unimplemented statement while parsing 'Expr'!");
        }
        Rule::sexpr => {
            let mut vals = vec![];

            for pair in pair.into_inner() {
                let val = read(pair)?;

                vals.push(val);
            }

            return Ok(Expr::Sexpr(vals));
        }
        Rule::qexpr => {
            let mut vals = vec![];

            for pair in pair.into_inner() {
                let val = read(pair)?;

                vals.push(val);
            }

            return Ok(Expr::Qexpr(vals));
        }
        Rule::atom => {
            for pair in pair.into_inner() {
                return read(pair);
            }

            // Really, this should never be reached
            return str_err!(
                "PARSING: Reached an unimplemented statement while parsing Expr for rule 'Atom'!"
            );
        }
        Rule::symbol => {
            let val = pair.as_str().to_ascii_lowercase();

            if get_builtin(val.clone()).is_some() {
                return Ok(Expr::Atom(Atom::Func(val)));
            }

            return Ok(Expr::Atom(Atom::Symbol(val)));
        }
        Rule::boolean => {
            let val = match pair.as_str().to_ascii_lowercase().as_str() {
                "true" => true,
                _ => false,
            };

            return Ok(Expr::Atom(Atom::Bool(val)));
        }
        Rule::string => {
            return Ok(Expr::Atom(Atom::Str(pair.as_str().into())));
        }
        Rule::number => {
            return Ok(Expr::Atom(Atom::Number(FixedNumber::from_str(
                pair.as_str().into(),
            ))));
        }
        _ => {
            return str_err!(
                "PARSING: Unhandled rule '{:?}' when reading!\nValue: {:?}",
                pair.as_rule(),
                pair,
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn env_get(key: String, env: &mut HashMap<String, Expr>) -> Result<Expr, String> {
    let val = env.get(&key);
    match val {
        Some(v) => {
            return Ok(v.clone());
        }
        None => {
            return str_err!("Undefined symbol {}!", key);
        }
    }
}

fn built_in_number_ops(
    op: NumberOp,
    others: Vec<Expr>,
    env: &mut HashMap<String, Expr>,
) -> Result<Expr, String> {
    // Evalutation: http://www.buildyourownlisp.com/chapter9_s_expressions

    // Validate all items are numbers; TODO: ADD TESTS
    let mut vals = vec![];

    for other in others {
        let val = eval(other, env)?;

        match val {
            Expr::Atom(Atom::Number(n)) => {
                vals.push(n);
            }
            _ => {
                return str_err!(
                    "EVAL: Symbol '{:?}' may only be applied to Numbers! Passed in {:?}",
                    op,
                    val
                );
            }
        }
    }

    let mut result = None;
    match op {
        NumberOp::Plus => {
            for val in vals {
                if result.is_none() {
                    result = Some(val);
                    continue;
                }

                let res = result.unwrap();

                result = Some(res + val);
            }
        }
        NumberOp::Minus => {
            if vals.len() == 1 {
                result = Some(-vals[0]);
            } else {
                for val in vals {
                    if result.is_none() {
                        result = Some(val);
                        continue;
                    }

                    let res = result.unwrap();

                    result = Some(res - val);
                }
            }
        }
        NumberOp::Multiply => {
            for val in vals {
                if result.is_none() {
                    result = Some(val);
                    continue;
                }

                let res = result.unwrap();

                result = Some(res * val);
            }
        }
        NumberOp::Divide => {
            for val in vals {
                if result.is_none() {
                    result = Some(val);
                    continue;
                }

                let res = result.unwrap();

                check!(val == 0.into(), ("EVAL: Attempted to divide by 0!"));
                result = Some(res / val);
            }
        }
    }

    check!(
        result.is_none(),
        ("EVAL: Attempted to use op '{:?}' on an empty!", op)
    );

    return Ok(Expr::Atom(Atom::Number(result.unwrap())));
}

#[derive(Debug, Clone, PartialEq)]
pub enum QexprOp {
    List,
    Head,
    Tail,
    Join,
    Eval,
    Def,
}

fn built_in_qexpr_op(
    op: QexprOp,
    others: Vec<Expr>,
    env: &mut HashMap<String, Expr>,
) -> Result<Expr, String> {
    match op {
        QexprOp::Head => {
            check!(
                others.len() != 1,
                (
                    "EVAL: Qexpr op '{:?}' passed {} args when expecting 1!",
                    op,
                    others.len()
                )
            );

            let other = others.first().unwrap();

            match other {
                Expr::Qexpr(vals) => {
                    check!(
                        vals.is_empty(),
                        (
                            "EVAL: Qexpr op '{:?}' requires at least one item in list!",
                            op
                        )
                    );

                    let first = vals.first().unwrap();
                    return Ok(Expr::Qexpr(vec![first.clone()]));
                }
                _ => {
                    let other = eval(other.clone(), env)?;

                    match other {
                        Expr::Qexpr(_) => {
                            return built_in_qexpr_op(QexprOp::Eval, vec![other], env);
                        }
                        _ => {
                            return str_err!(
                                "EVAL: Qexpr op '{:?}' not implemented for {:?}!",
                                op,
                                other
                            );
                        }
                    }
                }
            }
        }
        QexprOp::Tail => {
            check!(
                others.len() != 1,
                (
                    "EVAL: Qexpr op '{:?}' passed {} args when expecting 1!",
                    op,
                    others.len()
                )
            );

            let other = others.first().unwrap();

            match other {
                Expr::Qexpr(vals) => {
                    check!(
                        vals.is_empty(),
                        (
                            "EVAL: Qexpr op '{:?}' requires at least one item in list!",
                            op
                        )
                    );

                    let mut vals = vals.clone();
                    vals.remove(0);

                    return Ok(Expr::Qexpr(vals));
                }
                _ => {
                    return str_err!("EVAL: Qexpr op '{:?}' not implemented for {:?}!", op, other);
                }
            }
        }
        QexprOp::List => {
            let mut others = others.clone();
            if others.len() > 1 {
                others = vec![Expr::Sexpr(others)];
            }

            check!(
                others.is_empty() || others.len() != 1,
                (
                    "EVAL: Qexpr op '{:?}' passed {} args when expecting 1!",
                    op,
                    others.len()
                )
            );

            let other = others.first().unwrap();

            match other {
                Expr::Sexpr(vals) => {
                    return Ok(Expr::Qexpr(vals.clone()));
                }
                _ => {
                    return str_err!("EVAL: Qexpr op '{:?}' not implemented for {:?}!", op, other);
                }
            }
        }
        QexprOp::Eval => {
            check!(
                others.len() != 1,
                (
                    "EVAL: Qexpr op '{:?}' passed {} args when expecting 1!",
                    op,
                    others.len()
                )
            );

            let other = others.first().unwrap();

            match other {
                Expr::Qexpr(vals) => {
                    let expr = Expr::Sexpr(vals.clone());

                    return eval(expr, env);
                }
                _ => {
                    return eval(other.clone(), env);
                }
            }
        }
        QexprOp::Join => {
            let mut exprs = vec![];

            for other in others {
                match other {
                    Expr::Qexpr(vals) => {
                        exprs.append(&mut vals.clone());
                    }
                    _ => {
                        return str_err!(
                            "EVAL: Qexpr op '{:?}' not implemented for {:?}!",
                            op,
                            other
                        );
                    }
                }
            }

            return Ok(Expr::Qexpr(exprs));
        }
        QexprOp::Def => {
            const min_args: usize = 2;
            if others.len() < min_args {
                return str_err!("EXECUTION: 'def' requires at least {} arguments!", min_args);
            }

            let mut others = others.clone();

            let first = others.remove(0);

            let args = match first {
                Expr::Qexpr(vals) => {
                    let mut symbols = vec![];
                    for (i, val) in vals.iter().enumerate() {
                        match val {
                            Expr::Atom(Atom::Symbol(symbol)) => {
                                symbols.push(symbol.clone());
                            }
                            _ => {
                                return str_err!(
                                    "EXECUTION: 'def' passed a non-symbol definition at index {}!",
                                    i
                                );
                            }
                        }
                    }
                    Ok(symbols)
                }
                Expr::Atom(Atom::Symbol(s)) => {
                    let val = env_get(s, env)?;
                    let val = eval(val, env)?;
                    let mut symbols = vec![];

                    match val {
                        Expr::Qexpr(vals) => {
                            for (i, val) in vals.iter().enumerate() {
                                match val {
                                    Expr::Atom(Atom::Symbol(symbol)) => {
                                        symbols.push(symbol.clone());
                                    }
                                    _ => {
                                        return str_err!(
                                        "EXECUTION: 'def' passed a non-symbol definition at index {}!",
                                        i
                                    );
                                    }
                                }
                            }
                            Ok(symbols)
                        }
                        _ => str_err!("EXECUTION: 'def' not passed a Qexpr!"),
                    }
                }
                _ => str_err!("EXECUTION: 'def' not passed a Qexpr!"),
            }?;

            if args.is_empty() {
                //TODO: error
            }

            if args.len() != others.len() {
                //TODO: error
            }

            for (key, value) in args.iter().zip(others) {
                env_put(key.to_string(), value, env);
            }

            return Ok(Expr::Sexpr(vec![]));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltIn {
    Number(NumberOp),
    Qexpr(QexprOp),
}

fn get_builtin(symbol: String) -> Option<BuiltIn> {
    match symbol.as_str() {
        "+" => {
            return Some(BuiltIn::Number(NumberOp::Plus));
        }
        "-" => {
            return Some(BuiltIn::Number(NumberOp::Minus));
        }
        "*" => {
            return Some(BuiltIn::Number(NumberOp::Multiply));
        }
        "/" => {
            return Some(BuiltIn::Number(NumberOp::Divide));
        }
        "list" => {
            return Some(BuiltIn::Qexpr(QexprOp::List));
        }
        "head" => {
            return Some(BuiltIn::Qexpr(QexprOp::Head));
        }
        "tail" => {
            return Some(BuiltIn::Qexpr(QexprOp::Tail));
        }
        "join" => {
            return Some(BuiltIn::Qexpr(QexprOp::Join));
        }
        "eval" => {
            return Some(BuiltIn::Qexpr(QexprOp::Eval));
        }
        "def" => {
            return Some(BuiltIn::Qexpr(QexprOp::Def));
        }
        _ => {
            return None;
        }
    }
}

fn eval_builtin(
    op: BuiltIn,
    others: Vec<Expr>,
    env: &mut HashMap<String, Expr>,
) -> Result<Expr, String> {
    match op {
        BuiltIn::Number(op) => {
            return built_in_number_ops(op, others, env);
        }
        BuiltIn::Qexpr(op) => {
            return built_in_qexpr_op(op, others, env);
        }
    }
}

fn eval_symbol(
    symbol: Expr,
    others: Vec<Expr>,
    env: &mut HashMap<String, Expr>,
) -> Result<Expr, String> {
    match symbol {
        Expr::Atom(Atom::Func(s)) => {
            let val = get_builtin(s);

            match val.clone() {
                Some(b) => {
                    return eval_builtin(b, others, env);
                }
                None => {
                    return str_err!("EXECUTION: Unhandled func!");
                }
            }
        }
        Expr::Atom(Atom::Symbol(s)) => {
            let val = env.get(&s);

            match val {
                Some(v) => {
                    return Ok(v.clone());
                }
                None => {
                    return str_err!("EXECUTION: Unhandled GET '{:?}'!", s);
                }
            }
        }
        _ => {
            return str_err!("EXECUTION: Unhandled ENV Get '{:?}'!", symbol);
        }
    }
}

fn env_put(key: String, expr: Expr, env: &mut HashMap<String, Expr>) {
    env.insert(key, expr);
}

fn eval_sexpr(sexpr_vals: Vec<Expr>, env: &mut HashMap<String, Expr>) -> Result<Expr, String> {
    let mut vals = vec![];
    for val in sexpr_vals {
        let val = eval(val, env)?;
        vals.push(val);
    }

    if vals.is_empty() {
        return Ok(Expr::Sexpr(vals));
    }

    if vals.len() == 1 {
        return eval(vals[0].clone(), env);
    }

    // Ensure first element is symbol
    match vals[0] {
        Expr::Atom(Atom::Symbol(_)) => {}
        Expr::Atom(Atom::Func(_)) => {}
        _ => {
            return str_err!("EVAL: Sexpr doesn't start with a symbol/func!");
        }
    }

    return eval_symbol(vals.remove(0), vals, env);
}

fn eval(expr: Expr, env: &mut HashMap<String, Expr>) -> Result<Expr, String> {
    match expr.clone() {
        Expr::Atom(atom) => match atom {
            Atom::Symbol(s) => {
                return eval_symbol(expr, vec![], env);
            }
            _ => {
                return Ok(Expr::Atom(atom));
            }
        },
        Expr::Qexpr(atoms) => {
            return Ok(Expr::Qexpr(atoms));
        }
        Expr::Sexpr(atoms) => {
            return eval_sexpr(atoms, env);
        }
    }

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Slisp {
        let slisp = Slisp::new();
        return slisp;
    }

    fn num(num: FixedNumber) -> Expr {
        Expr::Atom(Atom::Number(num))
    }

    fn qexpr(exprs: Vec<Expr>) -> Expr {
        Expr::Qexpr(exprs)
    }

    fn sym(s: &'static str) -> Expr {
        Expr::Atom(Atom::Symbol(s.to_string()))
    }

    fn sexpr(exprs: Vec<Expr>) -> Expr {
        Expr::Sexpr(exprs)
    }

    fn func(s: &'static str) -> Expr {
        Expr::Atom(Atom::Func(s.to_string()))
    }

    #[test]
    fn Slisp_eval_functions_case1() {
        let mut slisp = init();
        let input = String::from("def {add-mul} (\\ {x y} {+ x (* x y)})");
        let expected = String::from("()");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();
        assert_eq!(expected, slisp.print(&actual));

        let input = String::from("add-mul 10 20");
        let expected = String::from("210.0");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();
        assert_eq!(expected, slisp.print(&actual));

        let input = String::from("add-mul 10");
        let expected = String::from("(\\ {y} {+ x (* x y)})");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();
        assert_eq!(expected, slisp.print(&actual));

        let input = String::from("def {add-mul-ten} (add-mul 10)");
        let expected = String::from("()");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();
        assert_eq!(expected, slisp.print(&actual));

        let input = String::from("add-mul-ten 50");
        let expected = String::from("510.0");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();
        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case12() {
        let mut slisp = init();

        let input = String::from("def {arglist} {a b x y}");
        slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        let input = String::from("def arglist 1 2 3 4");
        slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        let expected = String::from("{1.0 2.0 3.0 4.0}");

        let input = String::from("list a b x y");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case11() {
        let mut slisp = init();

        let input = String::from("def {a b} 5 6");
        slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        let expected = String::from("11.0");

        let input = String::from("+ a b");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case10() {
        let mut slisp = init();

        let input = String::from("def {x} 100");
        slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        let input = String::from("def {y} 200");
        slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        let expected = String::from("300.0");

        let input = String::from("+ x y");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case9() {
        let mut slisp = init();

        let expected = String::from("200.0");

        let input = String::from("def {y} 200");
        slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        let input = String::from("y");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case8() {
        let mut slisp = init();

        let expected = String::from("100.0");

        let input = String::from("def {x} 100");
        slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        let input = String::from("x");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case7() {
        let mut slisp = init();

        let expected = String::from("()");
        let input = String::from("def {y} 200");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case6() {
        let mut slisp = init();

        let expected = String::from("()");
        let input = String::from("def {x} 100");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case5() {
        let mut slisp = init();

        let expected = String::from("EXECUTION: Unhandled GET \'\"hello\"\'!");
        let input = String::from("hello");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_variables_case4() {
        let mut slisp = init();

        let expected = String::from("30.0");
        let input = String::from("(eval (head {+ - + - * /})) 10 20");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case3() {
        let mut slisp = init();

        let expected = String::from("+");
        let input = String::from("eval (head {+ - + - * /})");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case2() {
        let mut slisp = init();

        let expected = String::from("5.0");
        let input = String::from("eval (head {5 10 11 15})");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_variables_case1() {
        let mut slisp = init();

        let expected = func("+");
        let input = String::from("+");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_builtin_eval_head() {
        let mut slisp = init();

        let expected = num(3.into());
        let input = String::from("eval (head {(+ 1 2) (+ 10 20)})");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_builtin_eval_tail() {
        let mut slisp = init();

        let expected = qexpr(vec![num(6.into()), num(7.into())]);
        let input = String::from("eval (tail {tail tail {5 6 7}})");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_builtin_tail() {
        let mut slisp = init();

        let expected = String::from("{tail tail}");
        let input = String::from("tail {tail tail tail}");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_builtin_eval_list_head() {
        let mut slisp = init();

        let expected = String::from("{1.0}");
        let input = String::from("eval {head (list 1 2 3 4)}");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_builtin_join() {
        let mut slisp = init();

        let expected = String::from("{1.0 2.0 3.0 {4.0 5.0}}");
        let input = String::from("join {1 2} {3 {4 5}}");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_builtin_head_returns_qexpr() {
        let mut slisp = init();

        let expected = String::from("{1.0}");
        let input = String::from("head (list 1 2 3 4)");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_builtin_eval_list_case1() {
        let mut slisp = init();

        let expected = String::from("{1.0 2.0 3.0 4.0}");
        let input = String::from("(list 1 2 3 4)");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_builtin_list_head() {
        let mut slisp = init();

        let expected = String::from("{head (list 1.0 2.0 3.0 4.0)}");
        let input = String::from("{head (list 1 2 3 4)}");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_builtin_list() {
        let mut slisp = init();

        let expected = String::from("{1.0 2.0 3.0 4.0}");
        let input = String::from("list 1 2 3 4");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_qexpr_test_3() {
        let mut slisp = init();

        let expected = String::from("{{2.0 3.0 4.0} {1.0}}");
        let input = String::from("{{2 3 4} {1}}");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_qexpr_test_2() {
        let mut slisp = init();

        let expected = String::from("{1.0 2.0 (+ 5.0 6.0) 4.0}");
        let input = String::from("{1 2 (+ 5 6) 4}");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_qexpr_test_1() {
        let mut slisp = init();

        let expected = String::from("{1.0 2.0 3.0 4.0}");
        let input = String::from("{1 2 3 4}");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_divide_empty_sexpr_returns_err() {
        let mut slisp = init();
        let input = String::from("/ ()");
        let actual = slisp.eval(slisp.read_str(input).unwrap());
        let actual = actual.unwrap_err();

        assert_eq!(
            String::from(
                "EVAL: Symbol \'Divide\' may only be applied to Numbers! Passed in Sexpr([])"
            ),
            actual
        );
    }

    #[test]
    fn Slisp_eval_minus_one_num_returns_negative_num() {
        let mut slisp = init();

        let expected = Expr::Atom(Atom::Number((-100).into()));
        let input = String::from("(- 100)");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_complex_math_statement_returns_expected() {
        let mut slisp = init();

        let expected = String::from("39.0");

        let input = String::from("+ 1 (* 7 5) 3");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_divide_4_0_returns_err() {
        let mut slisp = init();

        let input = String::from("/ 4 0");
        let actual = slisp.eval(slisp.read_str(input).unwrap());

        let err = actual.unwrap_err();
        assert_eq!(format!("EVAL: Attempted to divide by 0!"), err);
    }

    #[test]
    fn Slisp_eval_sexpr_divide_8_4_2_returns_1() {
        let mut slisp = init();

        let expected = String::from("1.0");

        let input = String::from("/ 8 4 2");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_divide_0_returns_0() {
        let mut slisp = init();

        let expected = String::from("0.0");

        let input = String::from("/ 0");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_negative5_negative5_returns_25() {
        let mut slisp = init();

        let expected = String::from("25.0");

        let input = String::from("* -5 -5");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_negative5_returns_negative5() {
        let mut slisp = init();

        let expected = String::from("-5.0");

        let input = String::from("* -5");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_5_returns_5() {
        let mut slisp = init();

        let expected = String::from("5.0");

        let input = String::from("* 5");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_5_2_returns_10() {
        let mut slisp = init();

        let expected = String::from("10.0");

        let input = String::from("* 5 2");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_subtract_5_2_1_returns_2() {
        let mut slisp = init();

        let expected = String::from("2.0");

        let input = String::from("- 5 2 1");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_subtract_1_2_returns_negative1() {
        let mut slisp = init();

        let expected = String::from("-1.0");

        let input = String::from("- 1 2");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_subtract_1_returns_negative1() {
        let mut slisp = init();

        let expected = String::from("-1.0");

        let input = String::from("- 1");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_add_1_2_5_returns_8() {
        let mut slisp = init();

        let expected = String::from("8.0");

        let input = String::from("+ 1 2 5");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_add_1_2_returns_3() {
        let mut slisp = init();

        let expected = String::from("3.0");

        let input = String::from("+ 1 2");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_add_1_returns_1() {
        let mut slisp = init();

        let expected = String::from("1.0");

        let input = String::from("+ 1");
        let actual = slisp.eval(slisp.read_str(input).unwrap()).unwrap();

        assert_eq!(expected, slisp.print(&actual));
    }

    #[test]
    fn Slisp_eval_sexpr_single_atom_empty_returns_empty() {
        let mut env = default_env();

        let expected = Expr::Atom(Atom::Empty);
        let actual = eval(Expr::Sexpr(vec![expected.clone()]), &mut env).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_single_atom_number_returns_number() {
        let mut env = default_env();

        let expected = Expr::Atom(Atom::Number(1.into()));
        let actual = eval(Expr::Sexpr(vec![expected.clone()]), &mut env).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_empty_returns_empty_atom() {
        let mut env = default_env();

        let expected = Expr::Sexpr(vec![]);
        let actual = eval(Expr::Sexpr(vec![]), &mut env).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_empty_returns_empty_atom() {
        let mut env = default_env();

        let expected = Expr::Atom(Atom::Empty);
        let actual = eval(expected.clone(), &mut env).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_bool_returns_bool_atom() {
        let mut env = default_env();

        let expected = Expr::Atom(Atom::Bool(true));
        let actual = eval(expected.clone(), &mut env).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_number_returns_number_atom() {
        let mut env = default_env();

        let expected = Expr::Atom(Atom::Number(1.into()));
        let actual = eval(expected.clone(), &mut env).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_str_returns_str_atom() {
        let mut env = default_env();

        let expected = Expr::Atom(Atom::Str(format!("Hi there!")));
        let actual = eval(expected.clone(), &mut env).unwrap();
        assert_eq!(expected, actual);
    }
}
