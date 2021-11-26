#![feature(asm)]

fn main() {
    eprintln!("size_of::<usize> {:?}", std::mem::size_of::<usize>());

    let t = std::time::Instant::now();
    let i: u64 = 10u64.pow(9);
    let a: u64 = 1;
    let b: u64 = 1;
    let mut c: u64 = 0;
    unsafe {
        asm!(
            "92:",
            "mov {tmp:r}, {a:r}",
            "imul {tmp:r}, {b:r}",
            "imul {tmp:r}, {b:r}",
            "imul {tmp:r}, {b:r}",
            "imul {tmp:r}, {b:r}",
            "imul {tmp:r}, {b:r}",
            "add {c:r}, {tmp:r}",
            "sub {i:r}, 1",
            "jnz 92b",
            c = inout(reg) c,
            i = in(reg) i,
            a = in(reg) a,
            b = in(reg) b,
            tmp = out(reg) _,
        );
    }
    eprintln!("c = {:?}", c);
    eprintln!("{:?}", t.elapsed());

    let t = std::time::Instant::now();
    let i: u32 = 10u32.pow(9);
    let a: u32 = 1;
    let b: u32 = 1;
    let mut c: u32 = 0;
    unsafe {
        asm!(
            "92:",
            "mov {tmp:e}, {a:e}",
            "imul {tmp:e}, {b:e}",
            "imul {tmp:e}, {b:e}",
            "imul {tmp:e}, {b:e}",
            "imul {tmp:e}, {b:e}",
            "imul {tmp:e}, {b:e}",
            "add {c:e}, {tmp:e}",
            "sub {i:e}, 1",
            "jnz 92b",
            c = inout(reg) c,
            i = in(reg) i,
            a = in(reg) a,
            b = in(reg) b,
            tmp = out(reg) _,
        );
    }
    eprintln!("c = {:?}", c);
    eprintln!("{:?}", t.elapsed());
}
