#[macro_use] extern crate macrolisp;
use macrolisp::prelude::*;

#[test]
fn main() {
    let add = lisp!(
        (lambda ((a i32) (b i32) -> i32) (_add a b))
    );
    /*let factorial = lisp!(
        (lambda ((a i32) -> i32) (_if: (_eq: a 1)
                                       1
                                       (_mul: a (_recur: (_sub: a 1)))))
    );*/
    let factorial = lisp!(
        (lambda ((a i32) -> i32) (_let mut x a)
                                 (_let mut acc 1)
                                 (_while (_gt x 1)
                                  (_set acc (_mul acc x))
                                  (_set x   (_sub x 1)))
                                 acc)
    );
    
    println!("1+2+3+4 = {}", lisp!( (add 1 (add 2 (add 3 4))) ));
    println!("5! = {}", lisp!( (factorial 5) ));
    println!("10! = {}", lisp!( (factorial 10) ));
}

