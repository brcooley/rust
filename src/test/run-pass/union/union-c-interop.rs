// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(untagged_unions)]

#[derive(Clone, Copy)]
#[repr(C)]
struct LARGE_INTEGER_U {
    LowPart: u32,
    HighPart: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
union LARGE_INTEGER {
  __unnamed__: LARGE_INTEGER_U,
  u: LARGE_INTEGER_U,
  QuadPart: u64,
}

#[link(name = "rust_test_helpers")]
extern "C" {
    fn increment_all_parts(_: LARGE_INTEGER) -> LARGE_INTEGER;
}

fn main() {
    unsafe {
        let mut li = LARGE_INTEGER { QuadPart: 0 };
        let li_c = increment_all_parts(li);
        li.__unnamed__.LowPart += 1;
        li.__unnamed__.HighPart += 1;
        li.u.LowPart += 1;
        li.u.HighPart += 1;
        li.QuadPart += 1;
        assert_eq!(li.QuadPart, li_c.QuadPart);
    }
}
