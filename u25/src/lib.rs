#![no_std]
use crunchy::unroll;

use core as core_;

#[no_mangle]
extern "C" fn add(op1: U256, op2: U256) -> U256 {
    op1 + op2
}

use core::arch::wasm32::unreachable;
#[panic_handler]
fn on_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_map_from {
    ($thing:ident, $from:ty, $to:ty) => {
        impl From<$from> for $thing {
            fn from(value: $from) -> $thing {
                From::from(value as $to)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! uint_overflowing_binop {
    ($name:ident, $n_words: tt, $self_expr: expr, $other: expr, $fn:expr) => {{
        let $name(ref me) = $self_expr;
        let $name(ref you) = $other;

        let mut ret = [0u64; $n_words];
        let ret_ptr = &mut ret as *mut [u64; $n_words] as *mut u64;
        let mut carry = 0u64;

        // `unroll!` is recursive, but doesn’t use `$crate::unroll`, so we need to ensure that it
        // is in scope unqualified.
        use $crate::unroll;
        unroll! {
            for i in 0..$n_words {
                if carry != 0 {
                    let (res1, overflow1) = ($fn)(me[i], you[i]);
                    let (res2, overflow2) = ($fn)(res1, carry);

                    unsafe {
                        // SAFETY: `i` is within bounds and `i * size_of::<u64>() < isize::MAX`
                        *ret_ptr.offset(i as _) = res2
                    }
                    carry = (overflow1 as u8 + overflow2 as u8) as u64;
                } else {
                    let (res, overflow) = ($fn)(me[i], you[i]);

                    unsafe {
                        // SAFETY: `i` is within bounds and `i * size_of::<u64>() < isize::MAX`
                        *ret_ptr.offset(i as _) = res
                    }

                    carry = overflow as u64;
                }
            }
        }

        ($name(ret), carry > 0)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! uint_full_mul_reg {
    ($name:ident, 8, $self_expr:expr, $other:expr) => {
        $crate::uint_full_mul_reg!($name, 8, $self_expr, $other, |a, b| a != 0 || b != 0);
    };
    ($name:ident, $n_words:tt, $self_expr:expr, $other:expr) => {
        $crate::uint_full_mul_reg!($name, $n_words, $self_expr, $other, |_, _| true);
    };
    ($name:ident, $n_words:tt, $self_expr:expr, $other:expr, $check:expr) => {{
        {
            #![allow(unused_assignments)]

            let $name(ref me) = $self_expr;
            let $name(ref you) = $other;
            let mut ret = [0u64; $n_words * 2];

            use $crate::unroll;
            unroll! {
                for i in 0..$n_words {
                    let mut carry = 0u64;
                    let b = you[i];

                    unroll! {
                        for j in 0..$n_words {
                            if $check(me[j], carry) {
                                let a = me[j];

                                let (hi, low) = Self::split_u128(a as u128 * b as u128);

                                let overflow = {
                                    let existing_low = &mut ret[i + j];
                                    let (low, o) = low.overflowing_add(*existing_low);
                                    *existing_low = low;
                                    o
                                };

                                carry = {
                                    let existing_hi = &mut ret[i + j + 1];
                                    let hi = hi + overflow as u64;
                                    let (hi, o0) = hi.overflowing_add(carry);
                                    let (hi, o1) = hi.overflowing_add(*existing_hi);
                                    *existing_hi = hi;

                                    (o0 | o1) as u64
                                }
                            }
                        }
                    }
                }
            }

            ret
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! uint_overflowing_mul {
    ($name:ident, $n_words: tt, $self_expr: expr, $other: expr) => {{
        let ret: [u64; $n_words * 2] =
            $crate::uint_full_mul_reg!($name, $n_words, $self_expr, $other);

        // The safety of this is enforced by the compiler
        let ret: [[u64; $n_words]; 2] = unsafe { $crate::core_::mem::transmute(ret) };

        // The compiler WILL NOT inline this if you remove this annotation.
        #[inline(always)]
        fn any_nonzero(arr: &[u64; $n_words]) -> bool {
            use $crate::unroll;
            unroll! {
                for i in 0..$n_words {
                    if arr[i] != 0 {
                        return true;
                    }
                }
            }

            false
        }

        ($name(ret[0]), any_nonzero(&ret[1]))
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! overflowing {
    ($op: expr, $overflow: expr) => {{
        let (overflow_x, overflow_overflow) = $op;
        $overflow |= overflow_overflow;
        overflow_x
    }};
    ($op: expr) => {{
        let (overflow_x, _overflow_overflow) = $op;
        overflow_x
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! panic_on_overflow {
    ($name: expr) => {
        if $name {
            unreachable()
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_mul_from {
    ($name: ty, $other: ident) => {
        impl $crate::core_::ops::Mul<$other> for $name {
            type Output = $name;

            fn mul(self, other: $other) -> $name {
                let bignum: $name = other.into();
                let (result, overflow) = self.overflowing_mul(bignum);
                $crate::panic_on_overflow!(overflow);
                result
            }
        }

        impl<'a> $crate::core_::ops::Mul<&'a $other> for $name {
            type Output = $name;

            fn mul(self, other: &'a $other) -> $name {
                let bignum: $name = (*other).into();
                let (result, overflow) = self.overflowing_mul(bignum);
                $crate::panic_on_overflow!(overflow);
                result
            }
        }

        impl<'a> $crate::core_::ops::Mul<&'a $other> for &'a $name {
            type Output = $name;

            fn mul(self, other: &'a $other) -> $name {
                let bignum: $name = (*other).into();
                let (result, overflow) = self.overflowing_mul(bignum);
                $crate::panic_on_overflow!(overflow);
                result
            }
        }

        impl<'a> $crate::core_::ops::Mul<$other> for &'a $name {
            type Output = $name;

            fn mul(self, other: $other) -> $name {
                let bignum: $name = other.into();
                let (result, overflow) = self.overflowing_mul(bignum);
                $crate::panic_on_overflow!(overflow);
                result
            }
        }

        impl $crate::core_::ops::MulAssign<$other> for $name {
            fn mul_assign(&mut self, other: $other) {
                let result = *self * other;
                *self = result
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_mul_for_primitive {
    ($name: ty, $other: ident) => {
        impl $crate::core_::ops::Mul<$other> for $name {
            type Output = $name;

            fn mul(self, other: $other) -> $name {
                let (result, carry) = self.overflowing_mul_u64(other as u64);
                $crate::panic_on_overflow!(carry > 0);
                result
            }
        }

        impl<'a> $crate::core_::ops::Mul<&'a $other> for $name {
            type Output = $name;

            fn mul(self, other: &'a $other) -> $name {
                let (result, carry) = self.overflowing_mul_u64(*other as u64);
                $crate::panic_on_overflow!(carry > 0);
                result
            }
        }

        impl<'a> $crate::core_::ops::Mul<&'a $other> for &'a $name {
            type Output = $name;

            fn mul(self, other: &'a $other) -> $name {
                let (result, carry) = self.overflowing_mul_u64(*other as u64);
                $crate::panic_on_overflow!(carry > 0);
                result
            }
        }

        impl<'a> $crate::core_::ops::Mul<$other> for &'a $name {
            type Output = $name;

            fn mul(self, other: $other) -> $name {
                let (result, carry) = self.overflowing_mul_u64(other as u64);
                $crate::panic_on_overflow!(carry > 0);
                result
            }
        }

        impl $crate::core_::ops::MulAssign<$other> for $name {
            fn mul_assign(&mut self, other: $other) {
                let result = *self * (other as u64);
                *self = result
            }
        }
    };
}

#[macro_export]
macro_rules! construct_uint {
	( $(#[$attr:meta])* $visibility:vis struct $name:ident (1); ) => {
		$crate::construct_uint!{ @construct $(#[$attr])* $visibility struct $name (1); }
	};

	( $(#[$attr:meta])* $visibility:vis struct $name:ident ( $n_words:tt ); ) => {
			$crate::construct_uint! { @construct $(#[$attr])* $visibility struct $name ($n_words); }

			impl $crate::core_::convert::From<u128> for $name {
				fn from(value: u128) -> $name {
					let mut ret = [0; $n_words];
					ret[0] = value as u64;
					ret[1] = (value >> 64) as u64;
					$name(ret)
				}
			}

			impl $crate::core_::convert::From<i128> for $name {
				fn from(value: i128) -> $name {
					match value >= 0 {
						true => From::from(value as u128),
						false => { unreachable() }
					}
				}
			}

			impl $crate::core_::convert::TryFrom<$name> for u128 {
				type Error = &'static str;

				#[inline]
				fn try_from(u: $name) -> $crate::core_::result::Result<u128, &'static str> {
					let $name(arr) = u;
					for i in 2..$n_words {
						if arr[i] != 0 {
							return Err("integer overflow when casting to u128");
						}
					}
					Ok(((arr[1] as u128) << 64) + arr[0] as u128)
				}
			}

			impl $crate::core_::convert::TryFrom<$name> for i128 {
				type Error = &'static str;

				#[inline]
				fn try_from(u: $name) -> $crate::core_::result::Result<i128, &'static str> {
					let err_str = "integer overflow when casting to i128";
					let i = u128::try_from(u).map_err(|_| err_str)?;
					if i > i128::max_value() as u128 {
						Err(err_str)
					} else {
						Ok(i as i128)
					}
				}
			}
	};
	( @construct $(#[$attr:meta])* $visibility:vis struct $name:ident ( $n_words:tt ); ) => {
		/// Little-endian large integer type
		#[repr(C)]
		$(#[$attr])*
		#[derive(Copy, Clone, Eq, PartialEq, Hash)]
		$visibility struct $name ([u64; $n_words]);

		/// Get a reference to the underlying little-endian words.
		impl AsRef<[u64]> for $name {
			#[inline]
			fn as_ref(&self) -> &[u64] {
				&self.0
			}
		}

		impl<'a> From<&'a $name> for $name {
			fn from(x: &'a $name) -> $name {
				*x
			}
		}

		impl $name {
			const WORD_BITS: usize = 64;

			/// Low word (u64)
			#[inline]
			const fn low_u64(&self) -> u64 {
				let &$name(ref arr) = self;
				arr[0]
			}


			/// Conversion to usize with overflow checking
			///
			/// # Panics
			///
			/// Panics if the number is larger than usize::max_value().
			#[inline]
			fn as_usize(&self) -> usize {
				let &$name(ref arr) = self;
				if !self.fits_word() || arr[0] > usize::max_value() as u64 {
					unreachable()
				}
				arr[0] as usize
			}

			// Whether this fits u64.
			#[inline]
			fn fits_word(&self) -> bool {
				let &$name(ref arr) = self;
				for i in 1..$n_words { if arr[i] != 0 { return false; } }
				return true;
			}


			/// Return the least number of bits needed to represent the number
			#[inline]
			fn bits(&self) -> usize {
				let &$name(ref arr) = self;
				for i in 1..$n_words {
					if arr[$n_words - i] > 0 { return (0x40 * ($n_words - i + 1)) - arr[$n_words - i].leading_zeros() as usize; }
				}
				0x40 - arr[0].leading_zeros() as usize
			}
			/// Zero (additive identity) of this type.
			#[inline]
			const fn zero() -> Self {
				Self([0; $n_words])
			}

			fn full_shl(self, shift: u32) -> [u64; $n_words + 1] {
				let mut u = [0u64; $n_words + 1];
				let u_lo = self.0[0] << shift;
				let u_hi = self >> (Self::WORD_BITS as u32 - shift);
				u[0] = u_lo;
				u[1..].copy_from_slice(&u_hi.0[..]);
				u
			}

			fn full_shr(u: [u64; $n_words + 1], shift: u32) -> Self {
				let mut res = Self::zero();
				for i in 0..$n_words {
					res.0[i] = u[i] >> shift;
				}
				// carry
				if shift > 0 {
					for i in 1..=$n_words {
						res.0[i - 1] |= u[i] << (Self::WORD_BITS as u32 - shift);
					}
				}
				res
			}

			fn full_mul_u64(self, by: u64) -> [u64; $n_words + 1] {
				let (prod, carry) = self.overflowing_mul_u64(by);
				let mut res = [0u64; $n_words + 1];
				res[..$n_words].copy_from_slice(&prod.0[..]);
				res[$n_words] = carry;
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

			// See Knuth, TAOCP, Volume 2, section 4.3.1, Algorithm D.
			fn div_mod_knuth(self, mut v: Self, n: usize, m: usize) -> (Self, Self) {
				// D1.
				// Make sure 64th bit in v's highest word is set.
				// If we shift both self and v, it won't affect the quotient
				// and the remainder will only need to be shifted back.
				let shift = v.0[n - 1].leading_zeros();
				v <<= shift;
				// u will store the remainder (shifted)
				let mut u = self.full_shl(shift);

				// quotient
				let mut q = Self::zero();
				let v_n_1 = v.0[n - 1];
				let v_n_2 = v.0[n - 2];

				// D2. D7.
				// iterate from m downto 0
				for j in (0..=m).rev() {
					let u_jn = u[j + n];

					// D3.
					// q_hat is our guess for the j-th quotient digit
					// q_hat = min(b - 1, (u_{j+n} * b + u_{j+n-1}) / v_{n-1})
					// b = 1 << WORD_BITS
					// Theorem B: q_hat >= q_j >= q_hat - 2
					let mut q_hat = if u_jn < v_n_1 {
						let (mut q_hat, mut r_hat) = Self::div_mod_word(u_jn, u[j + n - 1], v_n_1);
						// this loop takes at most 2 iterations
						loop {
							// check if q_hat * v_{n-2} > b * r_hat + u_{j+n-2}
							let (hi, lo) = Self::split_u128(u128::from(q_hat) * u128::from(v_n_2));
							if (hi, lo) <= (r_hat, u[j + n - 2]) {
								break;
							}
							// then iterate till it doesn't hold
							q_hat -= 1;
							let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
							r_hat = new_r_hat;
							// if r_hat overflowed, we're done
							if overflow {
								break;
							}
						}
						q_hat
					} else {
						// here q_hat >= q_j >= q_hat - 1
						u64::max_value()
					};

					// ex. 20:
					// since q_hat * v_{n-2} <= b * r_hat + u_{j+n-2},
					// either q_hat == q_j, or q_hat == q_j + 1

					// D4.
					// let's assume optimistically q_hat == q_j
					// subtract (q_hat * v) from u[j..]
					let q_hat_v = v.full_mul_u64(q_hat);
					// u[j..] -= q_hat_v;
					let c = Self::sub_slice(&mut u[j..], &q_hat_v[..n + 1]);

					// D6.
					// actually, q_hat == q_j + 1 and u[j..] has overflowed
					// highly unlikely ~ (1 / 2^63)
					if c {
						q_hat -= 1;
						// add v to u[j..]
						let c = Self::add_slice(&mut u[j..], &v.0[..n]);
						u[j + n] = u[j + n].wrapping_add(u64::from(c));
					}

					// D5.
					q.0[j] = q_hat;
				}

				// D8.
				let remainder = Self::full_shr(u, shift);

				(q, remainder)
			}

			// Returns the least number of words needed to represent the nonzero number
			fn words(bits: usize) -> usize {
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

				if your_bits == 0 {unreachable()}

				// Early return in case we are dividing by a larger number than us
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
			fn overflowing_add(self, other: $name) -> ($name, bool) {
				$crate::uint_overflowing_binop!(
					$name,
					$n_words,
					self,
					other,
					u64::overflowing_add
				)
			}


			/// Subtraction which underflows and returns a flag if it does.
			#[inline(always)]
			fn overflowing_sub(self, other: $name) -> ($name, bool) {
				$crate::uint_overflowing_binop!(
					$name,
					$n_words,
					self,
					other,
					u64::overflowing_sub
				)
			}


			/// Multiply with overflow, returning a flag if it does.
			#[inline(always)]
			fn overflowing_mul(self, other: $name) -> ($name, bool) {
				$crate::uint_overflowing_mul!($name, $n_words, self, other)
			}

            #[inline(always)]
			fn div_mod_word(hi: u64, lo: u64, y: u64) -> (u64, u64) {
				// NOTE: this is slow (__udivti3)
				// let x = (u128::from(hi) << 64) + u128::from(lo);
				// let d = u128::from(d);
				// ((x / d) as u64, (x % d) as u64)
				// TODO: look at https://gmplib.org/~tege/division-paper.pdf
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

				let un21 = un32.wrapping_mul(TWO32).wrapping_add(un1).wrapping_sub(q1.wrapping_mul(y));
				let mut q0 = un21 / yn1;
				rhat = un21.wrapping_sub(q0.wrapping_mul(yn1));

				while q0 >= TWO32 || q0 * yn0 > TWO32 * rhat + un0 {
					q0 -= 1;
					rhat += yn1;
					if rhat >= TWO32 {
						break;
					}
				}

				let rem = un21.wrapping_mul(TWO32).wrapping_add(un0).wrapping_sub(y.wrapping_mul(q0));
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
			fn binop_slice(a: &mut [u64], b: &[u64], binop: impl Fn(u64, u64) -> (u64, bool) + Copy) -> bool {
				let mut c = false;
				a.iter_mut().zip(b.iter()).for_each(|(x, y)| {
					let (res, carry) = Self::binop_carry(*x, *y, c, binop);
					*x = res;
					c = carry;
				});
				c
			}

			#[inline(always)]
			fn binop_carry(a: u64, b: u64, c: bool, binop: impl Fn(u64, u64) -> (u64, bool)) -> (u64, bool) {
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

		impl $crate::core_::default::Default for $name {
			fn default() -> Self {
				$name::zero()
			}
		}

		impl $crate::core_::convert::From<u64> for $name {
			fn from(value: u64) -> $name {
				let mut ret = [0; $n_words];
				ret[0] = value;
				$name(ret)
			}
		}

		$crate::impl_map_from!($name, u8, u64);
		$crate::impl_map_from!($name, u16, u64);
		$crate::impl_map_from!($name, u32, u64);
		$crate::impl_map_from!($name, usize, u64);

		impl $crate::core_::convert::From<i64> for $name {
			fn from(value: i64) -> $name {
				match value >= 0 {
					true => From::from(value as u64),
					false => { unreachable() }
				}
			}
		}

		$crate::impl_map_from!($name, i8, i64);
		$crate::impl_map_from!($name, i16, i64);
		$crate::impl_map_from!($name, i32, i64);
		$crate::impl_map_from!($name, isize, i64);

		impl<T> $crate::core_::ops::Add<T> for $name where T: Into<$name> {
			type Output = $name;

			fn add(self, other: T) -> $name {
				let (result, overflow) = self.overflowing_add(other.into());
				$crate::panic_on_overflow!(overflow);
				result
			}
		}

		impl<'a, T> $crate::core_::ops::Add<T> for &'a $name where T: Into<$name> {
			type Output = $name;

			fn add(self, other: T) -> $name {
				*self + other
			}
		}

		impl $crate::core_::ops::AddAssign<$name> for $name {
			fn add_assign(&mut self, other: $name) {
				let (result, overflow) = self.overflowing_add(other);
				$crate::panic_on_overflow!(overflow);
				*self = result
			}
		}

		impl<T> $crate::core_::ops::Sub<T> for $name where T: Into<$name> {
			type Output = $name;

			#[inline]
			fn sub(self, other: T) -> $name {
				let (result, overflow) = self.overflowing_sub(other.into());
				$crate::panic_on_overflow!(overflow);
				result
			}
		}

		impl<'a, T> $crate::core_::ops::Sub<T> for &'a $name where T: Into<$name> {
			type Output = $name;

			fn sub(self, other: T) -> $name {
				*self - other
			}
		}

		impl $crate::core_::ops::SubAssign<$name> for $name {
			fn sub_assign(&mut self, other: $name) {
				let (result, overflow) = self.overflowing_sub(other);
				$crate::panic_on_overflow!(overflow);
				*self = result
			}
		}

		// all other impls
		$crate::impl_mul_from!($name, $name);
		$crate::impl_mul_for_primitive!($name, u8);
		$crate::impl_mul_for_primitive!($name, u16);
		$crate::impl_mul_for_primitive!($name, u32);
		$crate::impl_mul_for_primitive!($name, u64);
		$crate::impl_mul_for_primitive!($name, usize);
		$crate::impl_mul_for_primitive!($name, i8);
		$crate::impl_mul_for_primitive!($name, i16);
		$crate::impl_mul_for_primitive!($name, i32);
		$crate::impl_mul_for_primitive!($name, i64);
		$crate::impl_mul_for_primitive!($name, isize);

		impl<T> $crate::core_::ops::Div<T> for $name where T: Into<$name> {
			type Output = $name;

			fn div(self, other: T) -> $name {
				let other: Self = other.into();
				self.div_mod(other).0
			}
		}

		impl<'a, T> $crate::core_::ops::Div<T> for &'a $name where T: Into<$name> {
			type Output = $name;

			fn div(self, other: T) -> $name {
				*self / other
			}
		}

		impl<T> $crate::core_::ops::DivAssign<T> for $name where T: Into<$name> {
			fn div_assign(&mut self, other: T) {
				*self = *self / other.into();
			}
		}

		impl<T> $crate::core_::ops::Rem<T> for $name where T: Into<$name> + Copy {
			type Output = $name;

			fn rem(self, other: T) -> $name {
				let mut sub_copy = self;
				sub_copy %= other;
				sub_copy
			}
		}

		impl<'a, T> $crate::core_::ops::Rem<T> for &'a $name where T: Into<$name>  + Copy {
			type Output = $name;

			fn rem(self, other: T) -> $name {
				*self % other
			}
		}

		impl<T> $crate::core_::ops::RemAssign<T> for $name where T: Into<$name> + Copy {
			fn rem_assign(&mut self, other: T) {
				let other: Self = other.into();
				let rem = self.div_mod(other).1;
				*self = rem;
			}
		}

		impl $crate::core_::ops::BitAnd<$name> for $name {
			type Output = $name;

			#[inline]
			fn bitand(self, other: $name) -> $name {
				let $name(ref arr1) = self;
				let $name(ref arr2) = other;
				let mut ret = [0u64; $n_words];
				for i in 0..$n_words {
					ret[i] = arr1[i] & arr2[i];
				}
				$name(ret)
			}
		}

		impl $crate::core_::ops::BitXor<$name> for $name {
			type Output = $name;

			#[inline]
			fn bitxor(self, other: $name) -> $name {
				let $name(ref arr1) = self;
				let $name(ref arr2) = other;
				let mut ret = [0u64; $n_words];
				for i in 0..$n_words {
					ret[i] = arr1[i] ^ arr2[i];
				}
				$name(ret)
			}
		}

		impl $crate::core_::ops::BitOr<$name> for $name {
			type Output = $name;

			#[inline]
			fn bitor(self, other: $name) -> $name {
				let $name(ref arr1) = self;
				let $name(ref arr2) = other;
				let mut ret = [0u64; $n_words];
				for i in 0..$n_words {
					ret[i] = arr1[i] | arr2[i];
				}
				$name(ret)
			}
		}

		impl $crate::core_::ops::Not for $name {
			type Output = $name;

			#[inline]
			fn not(self) -> $name {
				let $name(ref arr) = self;
				let mut ret = [0u64; $n_words];
				for i in 0..$n_words {
					ret[i] = !arr[i];
				}
				$name(ret)
			}
		}

		impl<T> $crate::core_::ops::Shl<T> for $name where T: Into<$name> {
			type Output = $name;

			fn shl(self, shift: T) -> $name {
				let shift = shift.into().as_usize();
				let $name(ref original) = self;
				let mut ret = [0u64; $n_words];
				let word_shift = shift / 64;
				let bit_shift = shift % 64;

				// shift
				for i in word_shift..$n_words {
					ret[i] = original[i - word_shift] << bit_shift;
				}
				// carry
				if bit_shift > 0 {
					for i in word_shift+1..$n_words {
						ret[i] += original[i - 1 - word_shift] >> (64 - bit_shift);
					}
				}
				$name(ret)
			}
		}

		impl<'a, T> $crate::core_::ops::Shl<T> for &'a $name where T: Into<$name> {
			type Output = $name;
			fn shl(self, shift: T) -> $name {
				*self << shift
			}
		}

		impl<T> $crate::core_::ops::ShlAssign<T> for $name where T: Into<$name> {
			fn shl_assign(&mut self, shift: T) {
				*self = *self << shift;
			}
		}

		impl<T> $crate::core_::ops::Shr<T> for $name where T: Into<$name> {
			type Output = $name;

			fn shr(self, shift: T) -> $name {
				let shift = shift.into().as_usize();
				let $name(ref original) = self;
				let mut ret = [0u64; $n_words];
				let word_shift = shift / 64;
				let bit_shift = shift % 64;

				// shift
				for i in word_shift..$n_words {
					ret[i - word_shift] = original[i] >> bit_shift;
				}

				// Carry
				if bit_shift > 0 {
					for i in word_shift+1..$n_words {
						ret[i - word_shift - 1] += original[i] << (64 - bit_shift);
					}
				}

				$name(ret)
			}
		}

		impl<'a, T> $crate::core_::ops::Shr<T> for &'a $name where T: Into<$name> {
			type Output = $name;
			fn shr(self, shift: T) -> $name {
				*self >> shift
			}
		}

		impl<T> $crate::core_::ops::ShrAssign<T> for $name where T: Into<$name> {
			fn shr_assign(&mut self, shift: T) {
				*self = *self >> shift;
			}
		}

		impl $crate::core_::cmp::Ord for $name {
			fn cmp(&self, other: &$name) -> $crate::core_::cmp::Ordering {
				self.as_ref().iter().rev().cmp(other.as_ref().iter().rev())
			}
		}

		impl $crate::core_::cmp::PartialOrd for $name {
			fn partial_cmp(&self, other: &$name) -> Option<$crate::core_::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}
	}
}

#[cfg(not(feature = "arbitrary"))]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_arbitrary_for_uint {
    ($uint: ty, $n_bytes: tt) => {};
}

construct_uint! {
    /// 256-bit unsigned integer.
    #[cfg_attr(feature = "scale-info", derive(TypeInfo))]
    struct U256(4);
}
