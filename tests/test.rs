#![cfg_attr(feature = "nightly", feature(unboxed_closures, trace_macros))]

#[macro_use] extern crate macrolisp;
use macrolisp::prelude::*;

#[test]
fn main() {
    let add4 = lisp!(
        (lambda (((a i32)
                  (b i32)
                  (c i32)
                  (d i32))
                 i32)
         (+ a b c d))
    );
    let factorial_proc = lisp!(
        (lambda (((a i32))
                 i32)
         (let ((mut x   a)
                (mut acc 1))
          (while (> x 1)
           (:= acc (* acc x))
           (:= x   (- x 1)))
          acc))
    );
    lisp!(
        (defn factorial (((a i32))
                         i32)
         (if (== a 1)
             1
             (* a (factorial (- a 1)))))
    );

    lisp!(
        (defn factorial_tail (((a i32))
                              i32)
         (defn factorial_tail_helper (((a i32)
                                       (acc i32))
                                      i32)
          (if (== a 1)
              acc
              (factorial_tail_helper (- a 1) (* acc a))))
         (factorial_tail_helper a 1))
    );

    println!("1+2+3+4 = {}", lisp!( (add4 1 2 3 4) )); // TODO example with heterogeneous types
    println!("1-2-3-4 = {}", lisp!( (- 1 2 3 4) ));
    println!("5! = {}", lisp!( (factorial_proc 5) ));
    println!("6! = {}", lisp!( (factorial 6) ));
    println!("7! = {}", lisp!( (factorial_tail 7) ));
}

#[cfg(feature = "nightly")]
#[test]
fn nightly_tests() {
    let factorial_rec = lisp!(
        (lambda rec (((a i32))
                      i32)
         (if (== a 1)
             1
             (* a (rec (- a 1)))))
    );
    let fib = lisp!(
        (lambda rec (((a i32))
                      i32)
         (match a
          (0 1)
          (1 1)
          (n (+ (rec (- n 1))
                (rec (- n 2))))))
    );

    println!("-(8!) = {}", lisp!( (- (factorial_rec 8)) ));
    lisp!(
        (println! "fib = {} {} {} {} {} {} {} {} {} {} ..." (fib 0)
                                                            (fib 1)
                                                            (fib 2)
                                                            (fib 3)
                                                            (fib 4)
                                                            (fib 5)
                                                            (fib 6)
                                                            (fib 7)
                                                            (fib 8)
                                                            (fib 9))
    );
}

#[test]
fn lambda_tests() {
    let mut num = 5;
    lisp!(
        (let ((mut add_num (lambda (((x i32))
                                     ())
                             (:= num (+ num x)))))
         (add_num 5))
    );
    println!("num = {}", num);
}

