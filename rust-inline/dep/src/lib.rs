#![no_std]

#[cfg_attr(feature = "inline-foo", inline)]
pub fn foo() -> i32 {
    bar()
}

#[cfg_attr(feature = "inline-bar", inline)]
fn bar() -> i32 {
    90 + 2
}
