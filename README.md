# MacroLisp

## What?

A Lispy syntax for Rust.

## Why?

- Why not?
- To see if it was possible in the current macro system.
- [Greenspun's Tenth Rule](https://en.wikipedia.org/wiki/Greenspun%27s_tenth_rule). It was foreordained.

## How?

### Usage

See the [tests](tests/test.rs) for full usage examples.

To use MacroLisp, import the crate with the `#[macro_use]` attribute. You'll also want to bring in the prelude to get operators and other goodies:

```
#[macro_use] extern crate macrolisp;
use macrolisp::prelude::*;
```

All the public symbols in the prelude begin with an underscore, so conflicts should be minimal.

Some features in MacroLisp require experimental support from the compiler. If you want to use these features, you'll need to use a nightly rustc and turn on some features in your crate. This tables summarizes the requirements:

MacroLisp feature | Rust feature
------------------|-------------
Recursive lambdas | `#![feature(core, unboxed_closures)]`

### Implementation

One macro!

