pub mod prelude;

// TODO documentation
// TODO MACROS
#[macro_export] macro_rules! lisp {
    // empty
    () => (());
    (()) => (());

    // special forms
    ((lambda (($(($argn:ident $argt:ty))*) $ret:ty) $($body:tt)*)) => {
        // regular lambda
        |$($argn:$argt),*| -> $ret { $(lisp!($body));* }
    };
    ((lambda $s:ident (($(($argn:ident $argt:ty))*) $ret:ty) $($body:tt)*)) => {{
        // recursive lambda
        // $s MUST NOT be "self"
        // recurse by calling ($s ...)
        // FIXME recursive lambdas can't capture variables
        
        lisp!((_rust { fn $s($($argn: $argt),*) -> $ret { $(lisp!($body));* } $s }))
    }};
    ((defn $name:ident (($(($argn:ident $argt:ty))*) $ret:ty) $($body:tt)*)) => {
        fn $name($($argn:$argt),*) -> $ret { $(lisp!($body));* }
    };
    ((_if $cond:tt $yes:tt $no:tt)) => {
        if lisp!($cond) { lisp!($yes) } else { lisp!($no) }
    };
    ((_while $cond:tt $($body:tt)*)) => { // FIXME just one body tt, or move down to __LIST__ section to compile on stable
        while lisp!($cond) { $(lisp!($body));* }
    };
    // TODO for loops
    ((_match $var:tt $(($cond:tt $arm:tt))*)) => {
        match lisp!($var) {
            $(lisp!(__PAT__ $cond) => lisp!($arm)),*
        }
    };
    ((_do $($stmts:tt)*)) => {{ // FIXME is this necessary? (_let () ...) is the same
        $(lisp!($stmts));*
    }};

    // variables
    ((_let ((mut $var:ident $val:tt) $($bindings:tt)+) $($body:tt)*)) => {{
        let mut $var = lisp!($val);
        lisp!((_let ($($bindings)+) $($body)*))
    }};
    ((_let (($var:ident $val:tt) $($bindings:tt)+) $($body:tt)*)) => {{
        let $var = lisp!($val);
        lisp!((_let ($($bindings)+) $($body)*))
    }};
    ((_let ((mut $var:ident $val:tt)) $($body:tt)*)) => {{
        let mut $var = lisp!($val);
        $(lisp!($body));*
    }};
    ((_let ((mut $var:ident $val:tt)) $($body:tt)*)) => {{
        let mut $var = lisp!($val);
        $(lisp!($body));*
    }};
    ((_let (($var:ident $val:tt)) $($body:tt)*)) => {{
        let $var = lisp!($val);
        $(lisp!($body));*
    }};
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
    
    // parsers for unary and binary operators
    (__LIST__ -,    $arg:tt   ) => { lisp!(__UNARY_OP__  _neg,   $arg   ) };
    (__LIST__ !,    $arg:tt   ) => { lisp!(__UNARY_OP__  _not,   $arg   ) };
    (__LIST__ +,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _add, $($arg),*) };
    (__LIST__ &,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _and, $($arg),*) };
    (__LIST__ |,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _or,  $($arg),*) };
    (__LIST__ ^,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _xor, $($arg),*) };
    (__LIST__ /,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _div, $($arg),*) };
    (__LIST__ *,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _mul, $($arg),*) };
    (__LIST__ %,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _rem, $($arg),*) };
    (__LIST__ <<, $($arg:tt),*) => { lisp!(__BINARY_OP__ _shl, $($arg),*) };
    (__LIST__ >>, $($arg:tt),*) => { lisp!(__BINARY_OP__ _shr, $($arg),*) };
    (__LIST__ -,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _sub, $($arg),*) };
    (__LIST__ ==, $($arg:tt),*) => { lisp!(__BINARY_OP__ _eq,  $($arg),*) };
    (__LIST__ !=, $($arg:tt),*) => { lisp!(__BINARY_OP__ _ne,  $($arg),*) };
    (__LIST__ >,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _gt,  $($arg),*) };
    (__LIST__ <,  $($arg:tt),*) => { lisp!(__BINARY_OP__ _lt,  $($arg),*) };
    (__LIST__ >=, $($arg:tt),*) => { lisp!(__BINARY_OP__ _ge,  $($arg),*) };
    (__LIST__ <=, $($arg:tt),*) => { lisp!(__BINARY_OP__ _le,  $($arg),*) };

    // generically turn unary/binary operators into function calls
    // binary operators can be used as n-ary operators through __REDUCE__
    (__UNARY_OP__  $op:ident, $a:tt)        => { lisp!(__LIST__ $op, $a)      };
    (__BINARY_OP__ $op:ident, $a:tt, $b:tt) => { lisp!(__LIST__ $op, $a, $b)  };
    (__BINARY_OP__ $op:ident, $a:tt, $b:tt, $($rest:tt),+) =>
                                               { lisp!(__REDUCE__ $op,
                                                                  ($op $a $b),
                                                                  $($rest),+) };

    // reduce implementation
    // TODO external entry point for _reduce
    (__REDUCE__ $op:ident, $acc:tt)                       => { lisp!($acc)                     };
    (__REDUCE__ $op:ident, $acc:tt, $a:tt)                => { lisp!(__REDUCE__ $op,
                                                                                ($op $acc $a)) };
    (__REDUCE__ $op:ident, $acc:tt, $a:tt, $($rest:tt),+) => { lisp!(__REDUCE__ $op,
                                                                                ($op $acc $a),
                                                                                $($rest),+)    };

    // macro calls
    (__LIST__ $mac:ident, !) => {
        $mac!()
    };
    (__LIST__ $mac:ident, !, $($arg:tt),*) => {
        $mac!($(lisp!($arg)),*)
    };

    // function calls
    (__LIST__ $name:expr) => {
        lisp!($name)()
    };
    (__LIST__ $name:expr, $($arg:tt),*) => {
        lisp!($name)($(lisp!($arg)),*)
    };

    // one expression
    ($e:expr) => ($e);
    (__PAT__ $p:pat) => ($p);
}

