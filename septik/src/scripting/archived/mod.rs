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

pub struct Slisp {
    environment: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
pub enum Atom {
    Bool(bool),
    Number(FixedNumber),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(FixedNumber),
    Str(String),
    Operation(String),
    List(Vec<Value>),
}

fn eval_atom(pair: pest::iterators::Pair<Rule>) -> Result<Value, String> {
    match pair.as_rule() {
        Rule::number => {
            return Ok(Value::Number(FixedNumber::from_str(
                pair.as_str().to_string(),
            )));
        }
        Rule::boolean => match pair.as_str().to_ascii_lowercase().as_str() {
            "true" => return Ok(Value::Boolean(true)),
            _ => return Ok(Value::Boolean(false)),
        },
        Rule::string => {
            return Ok(Value::Str(pair.as_str().to_string()));
        }
        Rule::symbol => {
            return Ok(Value::Operation(pair.as_str().to_string()));
        }
        _ => {}
    }

    Err(format!(
        "ERROR: ATOM PASSED RULE '{:?}'!\n\t-> {}",
        pair.as_rule(),
        pair.as_str()
    ))
}

fn eval_op(op: String, values: Vec<Value>) -> Result<Value, String> {
    let mut final_val = None;
    for val in values {
        if final_val.is_none() {
            final_val = Some(val);
            continue;
        }

        match val {
            Value::Number(n) => {
                let mut v = final_val.unwrap();

                let f: Result<Value, String> = match v {
                    Value::Number(f) => {
                        match op.as_str() {
                            "+" => {
                                return Ok(Value::Number(n + f));
                            }
                            "-" => {
                                return Ok(Value::Number(n - f));
                            }
                            "*" => {
                                return Ok(Value::Number(n * f));
                            }
                            "/" => {
                                if f == 0.into() {
                                    return Err(format!("Attempt to divide by zero!"));
                                }

                                return Ok(Value::Number(n / f));
                            }
                            _ => {}
                        }
                        return Err(format!("Unhandled operation '{}'!", op));
                    }
                    _ => {
                        return Err(format!("Unhandled execution for rule '{:?}'!", v));
                    }
                };

                if f.is_ok() {
                    final_val = Some(f.unwrap());
                }
            }
            _ => {
                return Err(format!("Unhandled execution for rule '{:?}'!", val));
            }
        }
    }

    if final_val.is_some() {
        return Ok(final_val.unwrap());
    }

    return Err(format!("Unreacheable op!"));
}

impl Slisp {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    fn eval(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Value, String> {
        match pair.as_rule() {
            Rule::sexpr => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::atom => {
                            for pair in pair.into_inner() {
                                return eval_atom(pair);
                            }
                        }
                        Rule::list => {
                            return self.eval(pair);
                        }
                        _ => unimplemented!(),
                    }
                }
            }
            Rule::list => {
                let mut vals = vec![];

                for pair in pair.into_inner() {
                    let eval = self.eval(pair);
                    if eval.is_err() {
                        return eval;
                    }

                    vals.push(eval.unwrap());
                }

                if vals.is_empty() == false {
                    let val1 = vals.first().unwrap();

                    let mut op_type = None;
                    let is_op = match val1 {
                        Value::Operation(op) => {
                            op_type = Some(op);
                            true
                        }
                        _ => false,
                    };

                    if is_op && op_type.is_some() {
                        //TODO: Need to figure out how to execute ops
                        let op_type = op_type.unwrap();

                        // convert to stack
                        let mut vals = vals.clone();
                        vals.reverse();
                        let op = vals.pop();

                        return eval_op(op_type.to_string(), vals);
                    }
                }

                return Ok(Value::List(vals));
            }
            _ => {
                return Err(format!("ERROR: '{}'", pair.as_str()));
            }
        }

        Err(String::from("ERROR: NOT EXECUTED!"))
    }

    //TODO: move to return a result
    pub fn execute(&mut self, program: String) -> Result<Value, String> {
        let pairs = SlispParser::parse(Rule::program, &program);

        match pairs {
            Ok(pairs) => {
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::program => {
                            for pair in pair.into_inner() {
                                return self.eval(pair);
                            }
                        }
                        _ => unimplemented!("Not implemented for Rule '{}'!", pair),
                    }
                }
            }
            Err(error) => {
                return Err(format!("PARSING ERR: {}", error));
            }
        }

        Err(String::from("ERROR: UNREACHABLE CODE REACHED!"))
    }
}
