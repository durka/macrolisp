#![feature(trace_macros)]
trace_macros!(true);

macro_rules! define_binary_op {
    ($name:ident, $trt:ident, $func:ident) => {
        pub fn $name<A, B>(a: A, b: B) -> <A as ::std::ops::$trt<B>>::Output
            where A: ::std::ops::$trt<B>
        {
            a.$func(b)
        }
    };
    
    ($($name:ident, $trt:ident, $func:ident);*) => {
        $(define_binary_op!($name, $trt, $func);)*
    }
}

macro_rules! define_unary_op {
    ($name:ident, $trt:ident, $func:ident) => {
        pub fn $name<A>(a: A) -> <A as ::std::ops::$trt>::Output
            where A: ::std::ops::$trt
        {
            a.$func()
        }
    };
    
    ($($name:ident, $trt:ident, $func:ident);*) => {
        $(define_unary_op!($name, $trt, $func);)*
    }
}

define_binary_op!(_add, Add   , add;
                  _and, BitAnd, bitand;
                  _or , BitOr , bitor;
                  _xor, BitXor, bitxor;
                  _div, Div   , div;
                  _mul, Mul   , mul;
                  _rem, Rem   , rem;
                  _shl, Shl   , shl;
                  _shr, Shr   , shr;
                  _sub, Sub   , sub);
define_unary_op!( _neg, Neg   , neg;
                  _not, Not   , not);
                  
pub fn _eq<A, B>(a: A, b: B) -> bool
    where A: ::std::cmp::PartialEq<B>
{
    a == b
}

#[macro_export] macro_rules! lisp {
    // lambda
    ((lambda ($(($argn:ident $argt:ty))* -> $ret:ty) $body:tt)) => {
        |$($argn:$argt),*| -> $ret { lisp!($body) }
    };
    
    // special forms
    ((_if: $cond:tt $yes:tt $no:tt)) => {
        if lisp!($cond) { lisp!($yes) } else { lisp!($no) }
    };
    ((_while: $cond:tt $body:tt)) => {
        while lisp!($cond) { lisp!($body) }
    };
    ((_rust: $body:block)) => {
        { $body }
    };
    
    // call function
    (($name:expr)) => {
        (lisp!($name))()
    };
    (($name:path : $($arg:tt)*)) => {
        (lisp!($name))($(lisp!($arg)),*)
    };
    
    // one expression
    ($e:expr) => ($e);
    
    // empty
    () => (());
}

