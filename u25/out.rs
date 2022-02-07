#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use crunchy::unroll;
use core as core_;
#[no_mangle]
extern "C" fn div(op1: U256, op2: U256) -> U256 {
    if op2 == U256::zero() {
        U256::zero()
    } else {
        op1 / op2
    }
}
use core::arch::wasm32::unreachable;
#[panic_handler]
fn on_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
/// Little-endian large integer type
#[repr(C)]
/// 256-bit unsigned integer.
struct U256([u64; 4]);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for U256 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for U256 {
    #[inline]
    fn clone(&self) -> U256 {
        {
            let _: ::core::clone::AssertParamIsClone<[u64; 4]>;
            *self
        }
    }
}
impl ::core::marker::StructuralEq for U256 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for U256 {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<[u64; 4]>;
        }
    }
}
impl ::core::marker::StructuralPartialEq for U256 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for U256 {
    #[inline]
    fn eq(&self, other: &U256) -> bool {
        match *other {
            U256(ref __self_1_0) => match *self {
                U256(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &U256) -> bool {
        match *other {
            U256(ref __self_1_0) => match *self {
                U256(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for U256 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            U256(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
        }
    }
}
/// Get a reference to the underlying little-endian words.
impl AsRef<[u64]> for U256 {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        &self.0
    }
}
impl<'a> From<&'a U256> for U256 {
    fn from(x: &'a U256) -> U256 {
        *x
    }
}
impl U256 {
    const WORD_BITS: usize = 64;
    /// Low word (u64)
    #[inline]
    const fn low_u64(&self) -> u64 {
        let &U256(ref arr) = self;
        arr[0]
    }
    /// Conversion to usize with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than usize::max_value().
    #[inline]
    fn as_usize(&self) -> usize {
        let &U256(ref arr) = self;
        if !self.fits_word() || arr[0] > usize::max_value() as u64 {
            unreachable()
        }
        arr[0] as usize
    }
    #[inline]
    fn fits_word(&self) -> bool {
        let &U256(ref arr) = self;
        for i in 1..4 {
            if arr[i] != 0 {
                return false;
            }
        }
        return true;
    }
    /// Return the least number of bits needed to represent the number
    #[inline]
    fn bits(&self) -> usize {
        let &U256(ref arr) = self;
        for i in 1..4 {
            if arr[4 - i] > 0 {
                return (0x40 * (4 - i + 1)) - arr[4 - i].leading_zeros() as usize;
            }
        }
        0x40 - arr[0].leading_zeros() as usize
    }
    /// Zero (additive identity) of this type.
    #[inline]
    const fn zero() -> Self {
        Self([0; 4])
    }
    fn full_shl(self, shift: u32) -> [u64; 4 + 1] {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic("assertion failed: shift < Self::WORD_BITS as u32")
            };
        };
        let mut u = [0u64; 4 + 1];
        let u_lo = self.0[0] << shift;
        let u_hi = self >> (Self::WORD_BITS as u32 - shift);
        u[0] = u_lo;
        u[1..].copy_from_slice(&u_hi.0[..]);
        u
    }
    fn full_shr(u: [u64; 4 + 1], shift: u32) -> Self {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic("assertion failed: shift < Self::WORD_BITS as u32")
            };
        };
        let mut res = Self::zero();
        for i in 0..4 {
            res.0[i] = u[i] >> shift;
        }
        if shift > 0 {
            for i in 1..=4 {
                res.0[i - 1] |= u[i] << (Self::WORD_BITS as u32 - shift);
            }
        }
        res
    }
    fn full_mul_u64(self, by: u64) -> [u64; 4 + 1] {
        let (prod, carry) = self.overflowing_mul_u64(by);
        let mut res = [0u64; 4 + 1];
        res[..4].copy_from_slice(&prod.0[..]);
        res[4] = carry;
        res
    }
    fn div_mod_small(mut self, other: u64) -> (Self, Self) {
        let mut rem = 0u64;
        self.0.iter_mut().rev().for_each(|d| {
            let (q, r) = Self::div_mod_word(rem, *d, other);
            *d = q;
            rem = r;
        });
        (self, rem.into())
    }
    fn div_mod_knuth(self, mut v: Self, n: usize, m: usize) -> (Self, Self) {
        if true {
            if !(self.bits() >= v.bits() && !v.fits_word()) {
                ::core::panicking::panic(
                    "assertion failed: self.bits() >= v.bits() && !v.fits_word()",
                )
            };
        };
        if true {
            if !(n + m <= 4) {
                ::core::panicking::panic("assertion failed: n + m <= 4")
            };
        };
        let shift = v.0[n - 1].leading_zeros();
        v <<= shift;
        let mut u = self.full_shl(shift);
        let mut q = Self::zero();
        let v_n_1 = v.0[n - 1];
        let v_n_2 = v.0[n - 2];
        for j in (0..=m).rev() {
            let u_jn = u[j + n];
            let mut q_hat = if u_jn < v_n_1 {
                let (mut q_hat, mut r_hat) = Self::div_mod_word(u_jn, u[j + n - 1], v_n_1);
                loop {
                    let (hi, lo) = Self::split_u128(u128::from(q_hat) * u128::from(v_n_2));
                    if (hi, lo) <= (r_hat, u[j + n - 2]) {
                        break;
                    }
                    q_hat -= 1;
                    let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                    r_hat = new_r_hat;
                    if overflow {
                        break;
                    }
                }
                q_hat
            } else {
                u64::max_value()
            };
            let q_hat_v = v.full_mul_u64(q_hat);
            let c = Self::sub_slice(&mut u[j..], &q_hat_v[..n + 1]);
            if c {
                q_hat -= 1;
                let c = Self::add_slice(&mut u[j..], &v.0[..n]);
                u[j + n] = u[j + n].wrapping_add(u64::from(c));
            }
            q.0[j] = q_hat;
        }
        let remainder = Self::full_shr(u, shift);
        (q, remainder)
    }
    fn words(bits: usize) -> usize {
        if true {
            if !(bits > 0) {
                ::core::panicking::panic("assertion failed: bits > 0")
            };
        };
        1 + (bits - 1) / Self::WORD_BITS
    }
    /// Returns a pair `(self / other, self % other)`.
    ///
    /// # Panics
    ///
    /// Panics if `other` is zero.
    fn div_mod(self, other: Self) -> (Self, Self) {
        let my_bits = self.bits();
        let your_bits = other.bits();
        if your_bits == 0 {
            unreachable()
        }
        if my_bits < your_bits {
            return (Self::zero(), self);
        }
        if your_bits <= Self::WORD_BITS {
            return self.div_mod_small(other.low_u64());
        }
        let (n, m) = {
            let my_words = Self::words(my_bits);
            let your_words = Self::words(your_bits);
            (your_words, my_words - your_words)
        };
        self.div_mod_knuth(other, n, m)
    }
    /// Add with overflow.
    #[inline(always)]
    fn overflowing_add(self, other: U256) -> (U256, bool) {
        {
            let U256(ref me) = self;
            let U256(ref you) = other;
            let mut ret = [0u64; 4];
            let ret_ptr = &mut ret as *mut [u64; 4] as *mut u64;
            let mut carry = 0u64;
            use crate::unroll;
            #[allow(non_upper_case_globals)]
            #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 2;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 3;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U256(ret), carry > 0)
        }
    }
    /// Subtraction which underflows and returns a flag if it does.
    #[inline(always)]
    fn overflowing_sub(self, other: U256) -> (U256, bool) {
        {
            let U256(ref me) = self;
            let U256(ref you) = other;
            let mut ret = [0u64; 4];
            let ret_ptr = &mut ret as *mut [u64; 4] as *mut u64;
            let mut carry = 0u64;
            use crate::unroll;
            #[allow(non_upper_case_globals)]
            #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 2;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 3;
                    {
                        if i >= 0 {
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(me[i], you[i]);
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                unsafe { *ret_ptr.offset(i as _) = res2 }
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                unsafe { *ret_ptr.offset(i as _) = res }
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U256(ret), carry > 0)
        }
    }
    /// Multiply with overflow, returning a flag if it does.
    #[inline(always)]
    fn overflowing_mul(self, other: U256) -> (U256, bool) {
        {
            let ret: [u64; 4 * 2] = {
                {
                    #![allow(unused_assignments)]
                    let U256(ref me) = self;
                    let U256(ref you) = other;
                    let mut ret = [0u64; 4 * 2];
                    use crate::unroll;
                    #[allow(non_upper_case_globals)]
                    #[allow(unused_comparisons)]
                    {
                        {
                            const i: usize = 0;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 1;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 2;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 3;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) =
                                                            Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) =
                                                                low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) =
                                                                hi.overflowing_add(carry);
                                                            let (hi, o1) =
                                                                hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        };
                    }
                    ret
                }
            };
            let ret: [[u64; 4]; 2] = unsafe { crate::core_::mem::transmute(ret) };
            #[inline(always)]
            fn any_nonzero(arr: &[u64; 4]) -> bool {
                use crate::unroll;
                #[allow(non_upper_case_globals)]
                #[allow(unused_comparisons)]
                {
                    {
                        const i: usize = 0;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 1;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 2;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 3;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    };
                }
                false
            }
            (U256(ret[0]), any_nonzero(&ret[1]))
        }
    }
    #[inline(always)]
    fn div_mod_word(hi: u64, lo: u64, y: u64) -> (u64, u64) {
        if true {
            if !(hi < y) {
                ::core::panicking::panic("assertion failed: hi < y")
            };
        };
        const TWO32: u64 = 1 << 32;
        let s = y.leading_zeros();
        let y = y << s;
        let (yn1, yn0) = Self::split(y);
        let un32 = (hi << s) | lo.checked_shr(64 - s).unwrap_or(0);
        let un10 = lo << s;
        let (un1, un0) = Self::split(un10);
        let mut q1 = un32 / yn1;
        let mut rhat = un32 - q1 * yn1;
        while q1 >= TWO32 || q1 * yn0 > TWO32 * rhat + un1 {
            q1 -= 1;
            rhat += yn1;
            if rhat >= TWO32 {
                break;
            }
        }
        let un21 = un32
            .wrapping_mul(TWO32)
            .wrapping_add(un1)
            .wrapping_sub(q1.wrapping_mul(y));
        let mut q0 = un21 / yn1;
        rhat = un21.wrapping_sub(q0.wrapping_mul(yn1));
        while q0 >= TWO32 || q0 * yn0 > TWO32 * rhat + un0 {
            q0 -= 1;
            rhat += yn1;
            if rhat >= TWO32 {
                break;
            }
        }
        let rem = un21
            .wrapping_mul(TWO32)
            .wrapping_add(un0)
            .wrapping_sub(y.wrapping_mul(q0));
        (q1 * TWO32 + q0, rem >> s)
    }
    #[inline(always)]
    fn add_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_add)
    }
    #[inline(always)]
    fn sub_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_sub)
    }
    #[inline(always)]
    fn binop_slice(
        a: &mut [u64],
        b: &[u64],
        binop: impl Fn(u64, u64) -> (u64, bool) + Copy,
    ) -> bool {
        let mut c = false;
        a.iter_mut().zip(b.iter()).for_each(|(x, y)| {
            let (res, carry) = Self::binop_carry(*x, *y, c, binop);
            *x = res;
            c = carry;
        });
        c
    }
    #[inline(always)]
    fn binop_carry(
        a: u64,
        b: u64,
        c: bool,
        binop: impl Fn(u64, u64) -> (u64, bool),
    ) -> (u64, bool) {
        let (res1, overflow1) = b.overflowing_add(u64::from(c));
        let (res2, overflow2) = binop(a, res1);
        (res2, overflow1 || overflow2)
    }
    #[inline(always)]
    const fn mul_u64(a: u64, b: u64, carry: u64) -> (u64, u64) {
        let (hi, lo) = Self::split_u128(a as u128 * b as u128 + carry as u128);
        (lo, hi)
    }
    #[inline(always)]
    const fn split(a: u64) -> (u64, u64) {
        (a >> 32, a & 0xFFFF_FFFF)
    }
    #[inline(always)]
    const fn split_u128(a: u128) -> (u64, u64) {
        ((a >> 64) as _, (a & 0xFFFFFFFFFFFFFFFF) as _)
    }
    /// Overflowing multiplication by u64.
    /// Returns the result and carry.
    fn overflowing_mul_u64(mut self, other: u64) -> (Self, u64) {
        let mut carry = 0u64;
        for d in self.0.iter_mut() {
            let (res, c) = Self::mul_u64(*d, other, carry);
            *d = res;
            carry = c;
        }
        (self, carry)
    }
}
impl crate::core_::default::Default for U256 {
    fn default() -> Self {
        U256::zero()
    }
}
impl crate::core_::convert::From<u64> for U256 {
    fn from(value: u64) -> U256 {
        let mut ret = [0; 4];
        ret[0] = value;
        U256(ret)
    }
}
impl From<u8> for U256 {
    fn from(value: u8) -> U256 {
        From::from(value as u64)
    }
}
impl From<u16> for U256 {
    fn from(value: u16) -> U256 {
        From::from(value as u64)
    }
}
impl From<u32> for U256 {
    fn from(value: u32) -> U256 {
        From::from(value as u64)
    }
}
impl From<usize> for U256 {
    fn from(value: usize) -> U256 {
        From::from(value as u64)
    }
}
impl crate::core_::convert::From<i64> for U256 {
    fn from(value: i64) -> U256 {
        match value >= 0 {
            true => From::from(value as u64),
            false => unreachable(),
        }
    }
}
impl From<i8> for U256 {
    fn from(value: i8) -> U256 {
        From::from(value as i64)
    }
}
impl From<i16> for U256 {
    fn from(value: i16) -> U256 {
        From::from(value as i64)
    }
}
impl From<i32> for U256 {
    fn from(value: i32) -> U256 {
        From::from(value as i64)
    }
}
impl From<isize> for U256 {
    fn from(value: isize) -> U256 {
        From::from(value as i64)
    }
}
impl<T> crate::core_::ops::Add<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn add(self, other: T) -> U256 {
        let (result, overflow) = self.overflowing_add(other.into());
        if overflow {
            unreachable()
        };
        result
    }
}
impl<'a, T> crate::core_::ops::Add<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn add(self, other: T) -> U256 {
        *self + other
    }
}
impl crate::core_::ops::AddAssign<U256> for U256 {
    fn add_assign(&mut self, other: U256) {
        let (result, overflow) = self.overflowing_add(other);
        if overflow {
            unreachable()
        };
        *self = result
    }
}
impl<T> crate::core_::ops::Sub<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    #[inline]
    fn sub(self, other: T) -> U256 {
        let (result, overflow) = self.overflowing_sub(other.into());
        if overflow {
            unreachable()
        };
        result
    }
}
impl<'a, T> crate::core_::ops::Sub<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn sub(self, other: T) -> U256 {
        *self - other
    }
}
impl crate::core_::ops::SubAssign<U256> for U256 {
    fn sub_assign(&mut self, other: U256) {
        let (result, overflow) = self.overflowing_sub(other);
        if overflow {
            unreachable()
        };
        *self = result
    }
}
impl crate::core_::ops::Mul<U256> for U256 {
    type Output = U256;
    fn mul(self, other: U256) -> U256 {
        let bignum: U256 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a U256> for U256 {
    type Output = U256;
    fn mul(self, other: &'a U256) -> U256 {
        let bignum: U256 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a U256> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a U256) -> U256 {
        let bignum: U256 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<U256> for &'a U256 {
    type Output = U256;
    fn mul(self, other: U256) -> U256 {
        let bignum: U256 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<U256> for U256 {
    fn mul_assign(&mut self, other: U256) {
        let result = *self * other;
        *self = result
    }
}
impl crate::core_::ops::Mul<u8> for U256 {
    type Output = U256;
    fn mul(self, other: u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u8> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<u8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<u8> for U256 {
    fn mul_assign(&mut self, other: u8) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<u16> for U256 {
    type Output = U256;
    fn mul(self, other: u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u16> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<u16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<u16> for U256 {
    fn mul_assign(&mut self, other: u16) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<u32> for U256 {
    type Output = U256;
    fn mul(self, other: u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u32> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<u32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<u32> for U256 {
    fn mul_assign(&mut self, other: u32) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<u64> for U256 {
    type Output = U256;
    fn mul(self, other: u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u64> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a u64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<u64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<u64> for U256 {
    fn mul_assign(&mut self, other: u64) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<usize> for U256 {
    type Output = U256;
    fn mul(self, other: usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a usize> for U256 {
    type Output = U256;
    fn mul(self, other: &'a usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a usize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<usize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<usize> for U256 {
    fn mul_assign(&mut self, other: usize) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<i8> for U256 {
    type Output = U256;
    fn mul(self, other: i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i8> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<i8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<i8> for U256 {
    fn mul_assign(&mut self, other: i8) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<i16> for U256 {
    type Output = U256;
    fn mul(self, other: i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i16> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<i16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<i16> for U256 {
    fn mul_assign(&mut self, other: i16) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<i32> for U256 {
    type Output = U256;
    fn mul(self, other: i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i32> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<i32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<i32> for U256 {
    fn mul_assign(&mut self, other: i32) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<i64> for U256 {
    type Output = U256;
    fn mul(self, other: i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i64> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a i64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<i64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<i64> for U256 {
    fn mul_assign(&mut self, other: i64) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl crate::core_::ops::Mul<isize> for U256 {
    type Output = U256;
    fn mul(self, other: isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a isize> for U256 {
    type Output = U256;
    fn mul(self, other: &'a isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<&'a isize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl<'a> crate::core_::ops::Mul<isize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            unreachable()
        };
        result
    }
}
impl crate::core_::ops::MulAssign<isize> for U256 {
    fn mul_assign(&mut self, other: isize) {
        let result = *self * (other as u64);
        *self = result
    }
}
impl<T> crate::core_::ops::Div<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn div(self, other: T) -> U256 {
        let other: Self = other.into();
        self.div_mod(other).0
    }
}
impl<'a, T> crate::core_::ops::Div<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn div(self, other: T) -> U256 {
        *self / other
    }
}
impl<T> crate::core_::ops::DivAssign<T> for U256
where
    T: Into<U256>,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other.into();
    }
}
impl<T> crate::core_::ops::Rem<T> for U256
where
    T: Into<U256> + Copy,
{
    type Output = U256;
    fn rem(self, other: T) -> U256 {
        let mut sub_copy = self;
        sub_copy %= other;
        sub_copy
    }
}
impl<'a, T> crate::core_::ops::Rem<T> for &'a U256
where
    T: Into<U256> + Copy,
{
    type Output = U256;
    fn rem(self, other: T) -> U256 {
        *self % other
    }
}
impl<T> crate::core_::ops::RemAssign<T> for U256
where
    T: Into<U256> + Copy,
{
    fn rem_assign(&mut self, other: T) {
        let other: Self = other.into();
        let rem = self.div_mod(other).1;
        *self = rem;
    }
}
impl crate::core_::ops::BitAnd<U256> for U256 {
    type Output = U256;
    #[inline]
    fn bitand(self, other: U256) -> U256 {
        let U256(ref arr1) = self;
        let U256(ref arr2) = other;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = arr1[i] & arr2[i];
        }
        U256(ret)
    }
}
impl crate::core_::ops::BitXor<U256> for U256 {
    type Output = U256;
    #[inline]
    fn bitxor(self, other: U256) -> U256 {
        let U256(ref arr1) = self;
        let U256(ref arr2) = other;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = arr1[i] ^ arr2[i];
        }
        U256(ret)
    }
}
impl crate::core_::ops::BitOr<U256> for U256 {
    type Output = U256;
    #[inline]
    fn bitor(self, other: U256) -> U256 {
        let U256(ref arr1) = self;
        let U256(ref arr2) = other;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = arr1[i] | arr2[i];
        }
        U256(ret)
    }
}
impl crate::core_::ops::Not for U256 {
    type Output = U256;
    #[inline]
    fn not(self) -> U256 {
        let U256(ref arr) = self;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = !arr[i];
        }
        U256(ret)
    }
}
impl<T> crate::core_::ops::Shl<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shl(self, shift: T) -> U256 {
        let shift = shift.into().as_usize();
        let U256(ref original) = self;
        let mut ret = [0u64; 4];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..4 {
            ret[i] = original[i - word_shift] << bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..4 {
                ret[i] += original[i - 1 - word_shift] >> (64 - bit_shift);
            }
        }
        U256(ret)
    }
}
impl<'a, T> crate::core_::ops::Shl<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shl(self, shift: T) -> U256 {
        *self << shift
    }
}
impl<T> crate::core_::ops::ShlAssign<T> for U256
where
    T: Into<U256>,
{
    fn shl_assign(&mut self, shift: T) {
        *self = *self << shift;
    }
}
impl<T> crate::core_::ops::Shr<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shr(self, shift: T) -> U256 {
        let shift = shift.into().as_usize();
        let U256(ref original) = self;
        let mut ret = [0u64; 4];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..4 {
            ret[i - word_shift] = original[i] >> bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..4 {
                ret[i - word_shift - 1] += original[i] << (64 - bit_shift);
            }
        }
        U256(ret)
    }
}
impl<'a, T> crate::core_::ops::Shr<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shr(self, shift: T) -> U256 {
        *self >> shift
    }
}
impl<T> crate::core_::ops::ShrAssign<T> for U256
where
    T: Into<U256>,
{
    fn shr_assign(&mut self, shift: T) {
        *self = *self >> shift;
    }
}
impl crate::core_::cmp::Ord for U256 {
    fn cmp(&self, other: &U256) -> crate::core_::cmp::Ordering {
        self.as_ref().iter().rev().cmp(other.as_ref().iter().rev())
    }
}
impl crate::core_::cmp::PartialOrd for U256 {
    fn partial_cmp(&self, other: &U256) -> Option<crate::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl crate::core_::convert::From<u128> for U256 {
    fn from(value: u128) -> U256 {
        let mut ret = [0; 4];
        ret[0] = value as u64;
        ret[1] = (value >> 64) as u64;
        U256(ret)
    }
}
impl crate::core_::convert::From<i128> for U256 {
    fn from(value: i128) -> U256 {
        match value >= 0 {
            true => From::from(value as u128),
            false => unreachable(),
        }
    }
}
impl crate::core_::convert::TryFrom<U256> for u128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> crate::core_::result::Result<u128, &'static str> {
        let U256(arr) = u;
        for i in 2..4 {
            if arr[i] != 0 {
                return Err("integer overflow when casting to u128");
            }
        }
        Ok(((arr[1] as u128) << 64) + arr[0] as u128)
    }
}
impl crate::core_::convert::TryFrom<U256> for i128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> crate::core_::result::Result<i128, &'static str> {
        let err_str = "integer overflow when casting to i128";
        let i = u128::try_from(u).map_err(|_| err_str)?;
        if i > i128::max_value() as u128 {
            Err(err_str)
        } else {
            Ok(i as i128)
        }
    }
}
