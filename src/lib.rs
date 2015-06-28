pub mod prelude;

#[macro_export] macro_rules! lisp {
    // special forms
    ((lambda (($(($argn:ident $argt:ty))*) $ret:ty) $($body:tt)*)) => {
        // regular lambda
        |$($argn:$argt),*| -> $ret { $(lisp!($body));* }
    };
    ((lambda $s:ident (($(($argn:ident $argt:ty))*) $ret:ty) $($body:tt)*)) => {{
        // recursive lambda
        // $s MUST be "self"
        // recurse by calling (self ...)
        struct F;
        impl FnOnce<($($argt,)*)> for F {
            type Output = $ret;
            extern "rust-call" fn call_once($s, ($($argn,)*): ($($argt,)*)) -> $ret {
                $(lisp!($body));*
            }
        }
        impl FnMut<($($argt,)*)> for F {
            extern "rust-call" fn call_mut(&mut $s, ($($argn,)*): ($($argt,)*)) -> $ret {
                $(lisp!($body));*
            }
        }
        impl Fn<($($argt,)*)> for F {
            extern "rust-call" fn call(&$s, ($($argn,)*): ($($argt,)*)) -> $ret {
                $(lisp!($body));*
            }
        }
        F
    }};
    ((defn $name:ident (($(($argn:ident $argt:ty))*) $ret:ty) $($body:tt)*)) => {
        fn $name($($argn:$argt),*) -> $ret { $(lisp!($body));* }
    };
    ((_if $cond:tt $yes:tt $no:tt)) => {
        if lisp!($cond) { lisp!($yes) } else { lisp!($no) }
    };
    ((_while $cond:tt $($body:tt)*)) => {
        while lisp!($cond) { $(lisp!($body));* }
    };
    ((_match $var:tt $(($cond:tt $arm:tt))*)) => {
        match lisp!($var) {
            $(lisp!(__PAT__ $cond) => lisp!($arm)),*
        }
    };
    ((_prn $($arg:tt)*)) => {
        println!($(lisp!($arg)),*)
    };

    // variables
    ((_let mut $var:ident $val:tt)) => {
        let mut $var = lisp!($val);
    };
    ((_let $var:ident $val:tt)) => {
        let $var = lisp!($val);
    };
    ((_set $var:ident $val:tt)) => {
        $var = lisp!($val);
    };

    // escape hatch
    ((_rust $body:block)) => {
        { $body }
    };

    // list parsing
    (($($elem:tt)*)) => {
        lisp!(__LIST__ $($elem),*)
    };
    
    // operators and function calls
    (__LIST__ +,  $a:tt, $b:tt) => { lisp!(__LIST__ _add, $a, $b) };
    (__LIST__ &,  $a:tt, $b:tt) => { lisp!(__LIST__ _and, $a, $b) };
    (__LIST__ |,  $a:tt, $b:tt) => { lisp!(__LIST__ _or,  $a, $b) };
    (__LIST__ ^,  $a:tt, $b:tt) => { lisp!(__LIST__ _xor, $a, $b) };
    (__LIST__ /,  $a:tt, $b:tt) => { lisp!(__LIST__ _div, $a, $b) };
    (__LIST__ *,  $a:tt, $b:tt) => { lisp!(__LIST__ _mul, $a, $b) };
    (__LIST__ %,  $a:tt, $b:tt) => { lisp!(__LIST__ _rem, $a, $b) };
    (__LIST__ <<, $a:tt, $b:tt) => { lisp!(__LIST__ _shl, $a, $b) };
    (__LIST__ >>, $a:tt, $b:tt) => { lisp!(__LIST__ _shr, $a, $b) };
    (__LIST__ -,  $a:tt, $b:tt) => { lisp!(__LIST__ _sub, $a, $b) };
    (__LIST__ -,  $a:tt       ) => { lisp!(__LIST__ _neg, $a    ) };
    (__LIST__ !,  $a:tt       ) => { lisp!(__LIST__ _not, $a    ) };
    (__LIST__ ==, $a:tt, $b:tt) => { lisp!(__LIST__ _eq,  $a, $b) };
    (__LIST__ !=, $a:tt, $b:tt) => { lisp!(__LIST__ _ne,  $a, $b) };
    (__LIST__ >,  $a:tt, $b:tt) => { lisp!(__LIST__ _gt,  $a, $b) };
    (__LIST__ <,  $a:tt, $b:tt) => { lisp!(__LIST__ _lt,  $a, $b) };
    (__LIST__ >=, $a:tt, $b:tt) => { lisp!(__LIST__ _ge,  $a, $b) };
    (__LIST__ <=, $a:tt, $b:tt) => { lisp!(__LIST__ _le,  $a, $b) };
    (__LIST__ $name:expr) => {
        lisp!($name)()
    };
    (__LIST__ $name:expr, $($arg:tt),*) => {
        lisp!($name)($(lisp!($arg)),*)
    };

    // one expression
    ($e:expr) => ($e);
    (__PAT__ $p:pat) => ($p);
    
    // empty
    () => (());
}

