#![feature(core, unboxed_closures, trace_macros)]
//trace_macros!(true);

#[macro_use] extern crate macrolisp;
use macrolisp::prelude::*;

#[test]
fn main() {
    let add = lisp!(
        (lambda ((a i32) (b i32) -> i32) (_add a b))
    );
    let factorial_proc = lisp!(
        (lambda ((a i32) -> i32) (_let mut x a)
                                 (_let mut acc 1)
                                 (_while (_gt x 1)
                                  (_set acc (_mul acc x))
                                  (_set x   (_sub x 1)))
                                 acc)
    );
    lisp!(
        (defn factorial ((a i32) -> i32) (_if (_eq a 1)
                                              1
                                              (_mul a (factorial (_sub a 1)))))
    );
    let factorial_rec = lisp!(
        (lambda self ((a i32) -> i32) (_if (_eq a 1)
                                           1
                                           (_mul a (self (_sub a 1)))))
    );
    
    println!("1+2+3+4 = {}", lisp!( (add 1 (add 2 (add 3 4))) ));
    println!("5! = {}", lisp!( (factorial_proc 5) ));
    println!("6! = {}", lisp!( (factorial 6) ));
    println!("7! = {}", lisp!( (factorial_rec 7) ));
}

