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
    List(Vec<Atom>),
    Bool(bool),
    Number(FixedNumber),
    Str(String),
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Sexpr(Vec<Expr>),
    Qexpr(Vec<Expr>),
}

pub struct Slisp {
    environment: HashMap<String, bool>,
}

impl Slisp {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
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

    pub fn eval(&mut self, expr: Expr) -> Result<Expr, String> {
        return eval(expr);
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
enum NumberOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn built_in_number_ops(op: NumberOp, others: Vec<Expr>) -> Result<Expr, String> {
    // Evalutation: http://www.buildyourownlisp.com/chapter9_s_expressions

    // Validate all items are numbers; TODO: ADD TESTS
    let mut vals = vec![];

    for other in others {
        let val = eval(other)?;

        match val {
            Expr::Atom(Atom::Number(n)) => {
                vals.push(n);
            }
            _ => {
                return str_err!("EVAL: Symbol '{:?}' may only be applied to Numbers!", op);
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
enum QexprOp {
    List,
    Head,
    Tail,
    Join,
    Eval,
}

fn built_in_qexpr_op(op: QexprOp, others: Vec<Expr>) -> Result<Expr, String> {
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
                    return Ok(first.clone());
                }
                _ => {
                    return str_err!("EVAL: Qexpr op '{:?}' not implemented for {:?}!", op, other);
                }
            }
        }
        QexprOp::Tail => {
            check!(
                others.len() == 1,
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
                others.is_empty() || others.len() != 1,
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

                    return eval(expr);
                }
                _ => {
                    return str_err!("EVAL: Qexpr op '{:?}' not implemented for {:?}!", op, other);
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
    }
}

fn eval(expr: Expr) -> Result<Expr, String> {
    match expr {
        Expr::Atom(atom) => {
            return Ok(Expr::Atom(atom));
        }
        Expr::Qexpr(atoms) => {
            return Ok(Expr::Qexpr(atoms));
        }
        Expr::Sexpr(atoms) => {
            if atoms.is_empty() {
                return Ok(Expr::Atom(Atom::Empty));
            }

            if atoms.len() == 1 {
                let atom = eval(atoms.first().unwrap().clone())?;

                return Ok(atom);
            }

            let mut atoms = atoms.clone();

            let first = atoms.remove(0);

            let first = eval(first)?;

            match first {
                Expr::Atom(Atom::Symbol(s)) => match s.as_str() {
                    "+" => {
                        return built_in_number_ops(NumberOp::Plus, atoms);
                    }
                    "-" => {
                        return built_in_number_ops(NumberOp::Minus, atoms);
                    }
                    "*" => {
                        return built_in_number_ops(NumberOp::Multiply, atoms);
                    }
                    "/" => {
                        return built_in_number_ops(NumberOp::Divide, atoms);
                    }
                    "list" => {
                        return built_in_qexpr_op(QexprOp::List, atoms);
                    }
                    "head" => {
                        return built_in_qexpr_op(QexprOp::Head, atoms);
                    }
                    "tail" => {
                        return built_in_qexpr_op(QexprOp::Tail, atoms);
                    }
                    "join" => {
                        return built_in_qexpr_op(QexprOp::Join, atoms);
                    }
                    "eval" => {
                        return built_in_qexpr_op(QexprOp::Eval, atoms);
                    }
                    _ => {
                        return str_err!("EXECUTION: Unhandled symbol '{}'!", s);
                    }
                },
                _ => {
                    return str_err!("EXECUTION: Unhandled atom '{:?}'!", first);
                }
            }
            unimplemented!("What are you doing here?");
        }
    }

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Slisp_eval_sexpr_divide_4_0_returns_err() {
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("/"))),
            Expr::Atom(Atom::Number((8).into())),
            Expr::Atom(Atom::Number((4).into())),
            Expr::Atom(Atom::Number((0).into())),
        ]));

        assert_eq!(true, actual.is_err());

        let err = actual.unwrap_err();
        assert_eq!(format!("EVAL: Attempted to divide by 0!"), err);
    }

    #[test]
    fn Slisp_eval_sexpr_divide_8_4_2_returns_1() {
        let expected = Expr::Atom(Atom::Number((1).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("/"))),
            Expr::Atom(Atom::Number((8).into())),
            Expr::Atom(Atom::Number((4).into())),
            Expr::Atom(Atom::Number((2).into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_divide_0_returns_0() {
        let expected = Expr::Atom(Atom::Number((0).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("/"))),
            Expr::Atom(Atom::Number((0).into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_negative5_negative5_returns_25() {
        let expected = Expr::Atom(Atom::Number((25).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("*"))),
            Expr::Atom(Atom::Number((-5).into())),
            Expr::Atom(Atom::Number((-5).into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_negative5_returns_negative5() {
        let expected = Expr::Atom(Atom::Number((-5).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("*"))),
            Expr::Atom(Atom::Number((-5).into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_5_returns_5() {
        let expected = Expr::Atom(Atom::Number((5).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("*"))),
            Expr::Atom(Atom::Number(5.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_multiply_5_2_returns_10() {
        let expected = Expr::Atom(Atom::Number((10).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("*"))),
            Expr::Atom(Atom::Number(5.into())),
            Expr::Atom(Atom::Number(2.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_subtract_5_2_1_returns_2() {
        let expected = Expr::Atom(Atom::Number((2).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("-"))),
            Expr::Atom(Atom::Number(5.into())),
            Expr::Atom(Atom::Number(2.into())),
            Expr::Atom(Atom::Number(1.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_subtract_1_2_returns_negative1() {
        let expected = Expr::Atom(Atom::Number((-1).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("-"))),
            Expr::Atom(Atom::Number(1.into())),
            Expr::Atom(Atom::Number(2.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_subtract_1_returns_negative1() {
        let expected = Expr::Atom(Atom::Number((-1).into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("-"))),
            Expr::Atom(Atom::Number(1.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_add_1_2_5_returns_8() {
        let expected = Expr::Atom(Atom::Number(8.into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("+"))),
            Expr::Atom(Atom::Number(1.into())),
            Expr::Atom(Atom::Number(2.into())),
            Expr::Atom(Atom::Number(5.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_add_1_2_returns_3() {
        let expected = Expr::Atom(Atom::Number(3.into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("+"))),
            Expr::Atom(Atom::Number(1.into())),
            Expr::Atom(Atom::Number(2.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_add_1_returns_1() {
        let expected = Expr::Atom(Atom::Number(1.into()));
        let actual = eval(Expr::Sexpr(vec![
            Expr::Atom(Atom::Symbol(format!("+"))),
            Expr::Atom(Atom::Number(1.into())),
        ]))
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_single_atom_empty_returns_empty() {
        let expected = Expr::Atom(Atom::Empty);
        let actual = eval(Expr::Sexpr(vec![expected.clone()])).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_single_atom_number_returns_number() {
        let expected = Expr::Atom(Atom::Number(1.into()));
        let actual = eval(Expr::Sexpr(vec![expected.clone()])).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_sexpr_empty_returns_empty_atom() {
        let expected = Expr::Atom(Atom::Empty);
        let actual = eval(Expr::Sexpr(vec![])).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_empty_returns_empty_atom() {
        let expected = Expr::Atom(Atom::Empty);
        let actual = eval(expected.clone()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_list_returns_list_atom() {
        let expected = Expr::Atom(Atom::List(vec![]));
        let actual = eval(expected.clone()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_bool_returns_bool_atom() {
        let expected = Expr::Atom(Atom::Bool(true));
        let actual = eval(expected.clone()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_number_returns_number_atom() {
        let expected = Expr::Atom(Atom::Number(1.into()));
        let actual = eval(expected.clone()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_str_returns_str_atom() {
        let expected = Expr::Atom(Atom::Str(format!("Hi there!")));
        let actual = eval(expected.clone()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn Slisp_eval_single_atom_symbol_returns_symbol_atom() {
        let expected = Expr::Atom(Atom::Symbol(format!("Hi there!")));
        let actual = eval(expected.clone()).unwrap();
        assert_eq!(expected, actual);
    }
}
