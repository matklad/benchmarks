fn main() {
    eprintln!("cpu loop");
    time_cpu_loop();

    eprintln!("\nnoop call");
    time_noop_loop();
}

fn time_cpu_loop() {
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

    let jit = wasmer::Universal::new(wasmer::Singlepass::new());
    let store = wasmer::Store::new(&jit.engine());
    let bytes = &include_bytes!("../target/wasm32-unknown-unknown/release/benchmarks.wasm")[..];
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

    let env = wasm3::Environment::new().expect("Unable to create environment");
    let rt = env
        .create_runtime(1024 * 60)
        .expect("Unable to create runtime");
    let module = wasm3::Module::parse(&env, bytes).expect("Unable to parse module");

    let module = rt.load_module(module).expect("Unable to load module");
    let func = module
        .find_function::<(i64,), i64>("skewed_sum")
        .expect("Unable to find function");
    let t = std::time::Instant::now();
    let res = func.call(1_000_000).unwrap();
    println!("\nwasm3");
    println!("{:?}", res);
    println!("{:?}", t.elapsed());
}

fn time_noop_loop() {
    let n_iters = 1000;
    let bytes = &include_bytes!("../target/wasm32-unknown-unknown/release/benchmarks.wasm")[..];

    let t = std::time::Instant::now();
    let jit = wasmer::Universal::new(wasmer::Singlepass::new());
    let store = wasmer::Store::new(&jit.engine());
    for _ in 0..n_iters {
        let module = wasmer::Module::new(&store, bytes).unwrap();

        // The module doesn't import anything, so we create an empty import object.
        let import_object = wasmer::imports! {};
        let instance = wasmer::Instance::new(&module, &import_object).unwrap();

        let skewed_sum = instance.exports.get_function("skewed_sum").unwrap();
        let res = skewed_sum.call(&[wasmer::Value::I64(0)]).unwrap();
        assert_eq!(*res, [wasmer::Value::I64(0)]);
    }
    println!("\nwasm");
    println!("{:?}", t.elapsed());

    let env = wasm3::Environment::new().expect("Unable to create environment");
    let rt = env
        .create_runtime(1024 * 60)
        .expect("Unable to create runtime");

    let t = std::time::Instant::now();
    for _ in 0..n_iters {
        let module = wasm3::Module::parse(&env, bytes).expect("Unable to parse module");

        let module = rt.load_module(module).expect("Unable to load module");
        let func = module
            .find_function::<(i64,), i64>("skewed_sum")
            .expect("Unable to find function");
        let res = func.call(0).unwrap();
        assert_eq!(res, 0);
    }
    println!("\nwasm3");
    println!("{:?}", t.elapsed());
}
