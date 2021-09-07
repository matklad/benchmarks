Quick&stupid benchmark comparing wasmer and native compilation.

    $ cargo install bindgen
    $ cargo build --release --target wasm32-unknown-unknown
    $ cargo run --release --example bench
