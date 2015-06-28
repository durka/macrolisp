#![feature(trace_macros)]
//trace_macros!(true);

pub mod prelude;
                  
#[macro_export] macro_rules! lisp {
    // lambda
    ((lambda ($(($argn:ident $argt:ty))* -> $ret:ty) $($body:tt)*)) => {
        |$($argn:$argt),*| -> $ret { $(lisp!($body));* }
    };
    
    // special forms
    ((_if $cond:tt $yes:tt $no:tt)) => {
        if lisp!($cond) { lisp!($yes) } else { lisp!($no) }
    };
    ((_while $cond:tt $($body:tt)*)) => {
        while lisp!($cond) { $(lisp!($body));* }
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

