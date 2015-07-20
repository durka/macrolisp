#[macro_use] extern crate macrolisp;
use macrolisp::prelude::*;

macro_rules! ck {
    ($left:expr, $right:expr) => {{
        assert_eq!($left, $right);
        $left
    }}
}


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

    struct IntegralFloat { i: i32 }
    impl std::ops::Add for IntegralFloat {
        type Output = f32;
        fn add(self, rhs: Self) -> Self::Output {
            self.i as Self::Output + rhs.i as Self::Output
        }
    }

    println!("1+2+3+4 = {}",  ck!(lisp!( (add4 1 2 3 4)     ),                10));
    println!("1+2+3.0 = {}",  ck!(lisp!( (+ (rust { IntegralFloat { i: 1 } })
                                            (rust { IntegralFloat { i: 2 } })
                                            3.5) ),                           6.5));
    println!("1+2+3.0 = {}",  ck!(lisp!( (+ (IntegralFloat. (i 4) )
                                            (IntegralFloat. (i 5) )
                                            6.5) ),                           15.5));
    println!("1-2-3-4 = {}",  ck!(lisp!( (- 1 2 3 4)        ),                -8));
    println!("5! = {}",       ck!(lisp!( (factorial_proc 5) ),                120));
    println!("6! = {}",       ck!(lisp!( (factorial 6)      ),                720));
    println!("7! = {}",       ck!(lisp!( (factorial_tail 7) ),                5040));
}

#[test]
fn lambdarec_tests() {
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

    println!("-(8!) = {}", ck!(lisp!( (- (factorial_rec 8)) ), -40320));
    lisp!(
        (println! "fib = {} {} {} {} {} {} {} {} {} {} ..." (ck! (fib 0) 1)
                                                            (ck! (fib 1) 1)
                                                            (ck! (fib 2) 2)
                                                            (ck! (fib 3) 3)
                                                            (ck! (fib 4) 5)
                                                            (ck! (fib 5) 8)
                                                            (ck! (fib 6) 13)
                                                            (ck! (fib 7) 21)
                                                            (ck! (fib 8) 34)
                                                            (ck! (fib 9) 55))
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
    println!("num = {}", ck!(num, 10));
}

/* rust-lang/rust#12335

#[test] #[compile_fail]
fn lambdarec_cannot_capture() {
    let mut num = 5;
    lisp!(
        (let ((mut add_num (lambda rec (((x i32))
                                        ())
                            (if (> x 0)
                             (do
                              (:= num (+ num 1))
                              (rec (- x 1)))
                             ()))))
         (add_num 5))
    );
    println!("num = {}", ck!(num, 10));
}
*/

