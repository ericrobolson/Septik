#![no_std]
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "septik.pest"] // relative to src
struct MyParser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
