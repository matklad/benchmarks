fn main() -> std::io::Result<()> {
    let data: Vec<u8> = random_numbers()
        .flat_map(|it| it.to_le_bytes())
        .take(64 * 1024)
        .collect();
    std::fs::write("./input.data", data.as_slice())?;

    eprintln!("Rust + asm:");
    exec("cargo run -q --release --manifest-path ./rust/Cargo.toml")?;

    eprintln!("\nRust:");
    exec("cargo run -q --release --manifest-path ./rust/Cargo.toml --features=pure")?;

    eprintln!("\nZig + ReleaseFast");
    exec("zig run -O ReleaseFast ./zig/main.zig")?;

    eprintln!("\nZig + ReleaseSafe");
    exec("zig run -O ReleaseSafe ./zig/main.zig")?;

    Ok(())
}

fn random_numbers() -> impl Iterator<Item = u32> {
    let mut random = 92u32;
    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

fn exec(command: &str) -> std::io::Result<()> {
    let args = command.split_ascii_whitespace().collect::<Vec<_>>();
    let (cmd, args) = args.split_first().unwrap();
    let status = std::process::Command::new(cmd).args(args).status()?;
    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("command {:?} returned non-zero code", command),
        ));
    }
    Ok(())
}
