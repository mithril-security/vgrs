/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use std::arch::asm;

#[allow(unused_mut)]
#[inline(always)]
pub unsafe fn request(
        default: uint,
        request: uint,
        arg1: uint,
        arg2: uint,
        arg3: uint,
        arg4: uint,
        arg5: uint) -> usize {

    let args: [uint; 6] = [request, arg1, arg2, arg3, arg4, arg5];
    let mut result: uint;

    // Valgrind notices this magic instruction sequence and interprets
    // it as a kind of hypercall.  When not running under Valgrind,
    // the instructions do nothing and `default` is returned.
    asm!(
        "rol edi, {_3}",
        "rol edi, {_13}",
        "rol edi, {_61}",
        "rol edi, {_51}",
        "xchg ebx, ebx",

        _3 = const 3,
        _13 = const 13,
        _61 = const 61,
        _51 = const 51,
        in("eax") args.as_ptr(),
        inout("edx") default => result,
    );
    result
}
