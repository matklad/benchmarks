#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: u64| {
    mylib::panicky(data);
});
