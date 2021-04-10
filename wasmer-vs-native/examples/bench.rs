fn main() {
    {
        let status = std::process::Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "--lib",
                "--target=wasm32-unknown-unknown",
            ])
            .status()
            .unwrap();
        assert!(status.success());
    }

    let t = std::time::Instant::now();
    let res = benchmarks::skewed_sum(1_000_000);
    println!("native");
    println!("{}", res);
    println!("{:?}", t.elapsed());

    let jit = wasmer::JIT::new(wasmer::Singlepass::new());
    let store = wasmer::Store::new(&jit.engine());
    let bytes = include_bytes!("../target/wasm32-unknown-unknown/release/benchmarks.wasm");
    let module = wasmer::Module::new(&store, bytes).unwrap();

    // The module doesn't import anything, so we create an empty import object.
    let import_object = wasmer::imports! {};
    let instance = wasmer::Instance::new(&module, &import_object).unwrap();

    let skewed_sum = instance.exports.get_function("skewed_sum").unwrap();
    let t = std::time::Instant::now();
    let res = skewed_sum.call(&[wasmer::Value::I64(1_000_000)]).unwrap();
    println!("\nwasm");
    println!("{:?}", res);
    println!("{:?}", t.elapsed());
}
