use core::ops;
use core::cmp;

macro_rules! define_binary_op {
    ($name:ident, $md:ident::$trt:ident, $func:ident) => {
        pub fn $name<A, B>(a: A, b: B) -> <A as $md::$trt<B>>::Output
            where A: $md::$trt<B>
        {
            a.$func(b)
        }
    };

    ($($name:ident, $md:ident::$trt:ident, $func:ident);*) => {
        $(define_binary_op!($name, $md::$trt, $func);)*
    }
}

macro_rules! define_unary_op {
    ($name:ident, $md:ident::$trt:ident, $func:ident) => {
        pub fn $name<A>(a: A) -> <A as $md::$trt>::Output
            where A: $md::$trt
        {
            a.$func()
        }
    };

    ($($name:ident, $md:ident::$trt:ident, $func:ident);*) => {
        $(define_unary_op!($name, $md::$trt, $func);)*
    }
}

macro_rules! define_comparison_op {
    ($name:ident, $md:ident::$trt:ident, $func:ident) => {
        pub fn $name<A, B>(a: A, b: B) -> bool
            where A: $md::$trt<B>
        {
            a.$func(&b)
        }
    };

    ($($name:ident, $md:ident::$trt:ident, $func:ident);*) => {
        $(define_comparison_op!($name, $md::$trt, $func);)*
    }
}

define_binary_op!(_add, ops::Add   , add;
                  _and, ops::BitAnd, bitand;
                  _or , ops::BitOr , bitor;
                  _xor, ops::BitXor, bitxor;
                  _div, ops::Div   , div;
                  _mul, ops::Mul   , mul;
                  _rem, ops::Rem   , rem;
                  _shl, ops::Shl   , shl;
                  _shr, ops::Shr   , shr;
                  _sub, ops::Sub   , sub);
define_unary_op!( _neg, ops::Neg   , neg;
                  _not, ops::Not   , not);

define_comparison_op!(
                  _eq, cmp::PartialEq , eq;
                  _ne, cmp::PartialEq , ne;
                  _gt, cmp::PartialOrd, gt;
                  _lt, cmp::PartialOrd, lt;
                  _ge, cmp::PartialOrd, ge;
                  _le, cmp::PartialOrd, le);
