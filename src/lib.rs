#![no_std]

pub mod prelude;

// TODO documentation
// TODO MACROS
#[macro_export] macro_rules! lisp {
    // empty
    () => (());
    (()) => (());

    // patterns are special snowflakes
    (@pat $p:pat) => ($p);

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

        lisp!((rust { fn $s($($argn: $argt),*) -> $ret { $(lisp!($body));* } $s }))
    }};
    ((defn $name:ident (($(($argn:ident $argt:ty))*) $ret:ty) $($body:tt)*)) => {
        fn $name($($argn:$argt),*) -> $ret { $(lisp!($body));* }
    };
    ((if $cond:tt $yes:tt $no:tt)) => {
        if lisp!($cond) { lisp!($yes) } else { lisp!($no) }
    };
    (@list while, $cond:tt, $($body:tt),*) => {
        while lisp!($cond) { $(lisp!($body));* }
    };
    // TODO for loops
    (@list match, $var:tt, $(($cond:tt $arm:tt)),*) => {
        match lisp!($var) {
            $(lisp!(@pat $cond) => lisp!($arm)),*
        }
    };
    ((do $($stmts:tt)*)) => {{
        $(lisp!($stmts));*
    }};

    // variables
    ((let ((mut $var:ident $val:tt) $($bindings:tt)+) $($body:tt)*)) => {{
        let mut $var = lisp!($val);
        lisp!((let ($($bindings)+) $($body)*))
    }};
    ((let (($var:ident $val:tt) $($bindings:tt)+) $($body:tt)*)) => {{
        let $var = lisp!($val);
        lisp!((let ($($bindings)+) $($body)*))
    }};
    ((let ((mut $var:ident $val:tt)) $($body:tt)*)) => {{
        let mut $var = lisp!($val);
        $(lisp!($body));*
    }};
    ((let ((mut $var:ident $val:tt)) $($body:tt)*)) => {{
        let mut $var = lisp!($val);
        $(lisp!($body));*
    }};
    ((let (($var:ident $val:tt)) $($body:tt)*)) => {{
        let $var = lisp!($val);
        $(lisp!($body));*
    }};
    ((:= $var:ident $val:tt)) => {
        $var = lisp!($val);
    };

    // escape hatch
    ((rust $body:block)) => {
        { $body }
    };

    // list parsing
    (($($elem:tt)*)) => {
        lisp!(@list $($elem),*)
    };

    // parsers for unary and binary operators
    (@list -,    $arg:tt   ) => { lisp!(@unary  _neg,   $arg   ) };
    (@list !,    $arg:tt   ) => { lisp!(@unary  _not,   $arg   ) };
    (@list +,  $($arg:tt),*) => { lisp!(@binary _add, $($arg),*) };
    (@list &,  $($arg:tt),*) => { lisp!(@binary _and, $($arg),*) };
    (@list |,  $($arg:tt),*) => { lisp!(@binary _or,  $($arg),*) };
    (@list ^,  $($arg:tt),*) => { lisp!(@binary _xor, $($arg),*) };
    (@list /,  $($arg:tt),*) => { lisp!(@binary _div, $($arg),*) };
    (@list *,  $($arg:tt),*) => { lisp!(@binary _mul, $($arg),*) };
    (@list %,  $($arg:tt),*) => { lisp!(@binary _rem, $($arg),*) };
    (@list <<, $($arg:tt),*) => { lisp!(@binary _shl, $($arg),*) };
    (@list >>, $($arg:tt),*) => { lisp!(@binary _shr, $($arg),*) };
    (@list -,  $($arg:tt),*) => { lisp!(@binary _sub, $($arg),*) };
    (@list ==, $($arg:tt),*) => { lisp!(@binary _eq,  $($arg),*) };
    (@list !=, $($arg:tt),*) => { lisp!(@binary _ne,  $($arg),*) };
    (@list >,  $($arg:tt),*) => { lisp!(@binary _gt,  $($arg),*) };
    (@list <,  $($arg:tt),*) => { lisp!(@binary _lt,  $($arg),*) };
    (@list >=, $($arg:tt),*) => { lisp!(@binary _ge,  $($arg),*) };
    (@list <=, $($arg:tt),*) => { lisp!(@binary _le,  $($arg),*) };

    // generically turn unary/binary operators into function calls
    // binary operators can be used as n-ary operators through @reduce
    (@unary  $op:ident, $a:tt)        => { lisp!(@list $op, $a)     };
    (@binary $op:ident, $a:tt, $b:tt) => { lisp!(@list $op, $a, $b) };
    (@binary $op:ident, $a:tt, $b:tt, $($rest:tt),+) =>
                                               { lisp!(@reduce $op,
                                                       ($op $a $b),
                                                       $($rest),+) };

    // reduce implementation
    // TODO external entry point for @reduce
    (@reduce $op:ident, $acc:tt)                       => { lisp!($acc)          };
    (@reduce $op:ident, $acc:tt, $a:tt)                => { lisp!(@reduce $op,
                                                                  ($op $acc $a)) };
    (@reduce $op:ident, $acc:tt, $a:tt, $($rest:tt),+) => { lisp!(@reduce $op,
                                                                  ($op $acc $a),
                                                                  $($rest),+)    };

    // macro calls
    (@list $mac:ident, !) => {
        $mac!()
    };
    (@list $mac:ident, !, $($arg:tt),*) => {
        $mac!($(lisp!($arg)),*)
    };

    // struct constructors
    (@list $name:ident, ., $(($member:ident $val:tt)),*) => {
        $name { $($member: lisp!($val))* }
    };

    // function calls
    (@list $name:expr) => {
        lisp!($name)()
    };
    (@list $name:expr, $($arg:tt),*) => {
        lisp!($name)($(lisp!($arg)),*)
    };

    // one expression
    ($e:expr) => ($e);
}
