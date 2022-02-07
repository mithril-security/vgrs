/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_name="vgrs_memcheck_test"]
#![crate_type="bin"]
#![feature(libc)]
#![feature(test)]
#![deny(warnings)]

extern crate vgrs;
extern crate test;
extern crate libc;

use vgrs::{valgrind, memcheck};

use std::mem;
use libc::c_void;
use test::black_box;

unsafe fn assert_error(errors: &mut usize) {
    let e = valgrind::count_errors();
    assert!(e > *errors);
    *errors = e;
}

unsafe fn assert_no_error(errors: usize) {
    let e = valgrind::count_errors();
    assert_eq!(e, errors);
}

unsafe fn do_test() {
    assert_eq!(valgrind::running_on_valgrind(), 1);

    let mut errors = 0;
    assert_no_error(errors);

    let x: u8 = mem::MaybeUninit::uninit().assume_init();
    assert!(memcheck::check_is_addressable(&x).is_none());
    assert!(memcheck::check_is_defined(&x).is_some());
    black_box(x);
    assert_error(&mut errors);

    // Noaccess memory stays noaccess
    let mut x: u8 = 0;
    memcheck::make_noaccess(&x);
    assert!(memcheck::check_is_addressable(&x).is_some());
    assert!(memcheck::check_is_defined(&x).is_some());
    black_box(x);
    assert_error(&mut errors);
    x = 1;
    assert_error(&mut errors);
    black_box(x);
    assert_error(&mut errors);

    // Undefined memory becomes defined after a write
    let mut x: u8 = 0;
    memcheck::make_undefined(&x);
    assert!(memcheck::check_is_addressable(&x).is_none());
    assert!(memcheck::check_is_defined(&x).is_some());
    black_box(x);
    assert_error(&mut errors);
    x = 1;
    assert!(memcheck::check_is_addressable(&x).is_none());
    assert!(memcheck::check_is_defined(&x).is_none());
    assert_no_error(errors);
    black_box(x);
    assert_no_error(errors);

    let x: u8 = mem::MaybeUninit::uninit().assume_init();
    memcheck::make_defined(&x);
    assert!(memcheck::check_is_addressable(&x).is_none());
    assert!(memcheck::check_is_defined(&x).is_none());
    black_box(x);
    assert_no_error(errors);

    let mut x: *mut c_void = libc::malloc(42);
    assert_eq!(memcheck::count_leaks().leaked, 0);
    memcheck::do_leak_check();
    assert_eq!(memcheck::count_leaks().leaked, 0);

    // Make the malloc'd pointer live to here, then leak it
    black_box(&x);
    x = libc::malloc(0);

    assert_eq!(memcheck::count_leaks().leaked, 0);
    assert_eq!(memcheck::count_leak_blocks().leaked, 0);
    assert_no_error(errors);

    memcheck::do_leak_check();
    assert_error(&mut errors);
    assert_eq!(memcheck::count_leaks().leaked, 42);
    assert_eq!(memcheck::count_leak_blocks().leaked, 1);

    // Make sure the above assignment isn't dead
    black_box(&x);
}

fn main() {
    unsafe { do_test() }
}
