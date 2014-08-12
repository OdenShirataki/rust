// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:trait_bounds_on_structs_and_enums_xc.rs

extern crate trait_bounds_on_structs_and_enums_xc;

use trait_bounds_on_structs_and_enums_xc::{Bar, Foo, Trait};

fn explode(x: Foo<uint>) {}
//~^ ERROR failed to find an implementation
//~^^ ERROR instantiating a type parameter with an incompatible type

fn kaboom(y: Bar<f32>) {}
//~^ ERROR failed to find an implementation
//~^^ ERROR instantiating a type parameter with an incompatible type

fn main() {
    let foo = Foo {
    //~^ ERROR failed to find an implementation
    //~^^ ERROR instantiating a type parameter with an incompatible type
        x: 3i
    };
    let bar: Bar<f64> = return;
    //~^ ERROR failed to find an implementation
    //~^^ ERROR instantiating a type parameter with an incompatible type
    let _ = bar;
}

