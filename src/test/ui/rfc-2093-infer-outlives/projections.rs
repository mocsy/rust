// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-tidy-linelength

// Needs an explicit where clause stating outlives condition. RFC 2093

// Associated type <Iterator>::Item  needs to outlives lifetime 'a.
struct Foo<'a, T: Iterator> {
    bar: &'a T::Item //~ Error the associated type `<T as std::iter::Iterator>::Item` may not live long enough [E0309]
}

fn main() { }