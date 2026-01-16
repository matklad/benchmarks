const std = @import("std");
const assert = std.debug.assert;
const os = std.os;
const linux = std.os.linux;

const KiB: usize = 1024;
const GiB: usize = 1024 * 1024 * 1024;

const MAP_SIZE: usize = 16 * GiB;
const PAGE_SIZE: usize = 4 * KiB;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    var lock: struct {
        current: bool = false,
        future: bool = false,
        on_fault: bool = false,
    } = .{};

    for (args) |arg| {
        if (std.mem.eql(u8, arg, "--lock-current")) {
            lock.current = true;
        } else if (std.mem.eql(u8, arg, "--lock-future")) {
            lock.future = true;
        } else if (std.mem.eql(u8, arg, "--lock-on-fault")) {
            lock.on_fault = true;
        }
    }

    var timer = try std.time.Timer.start();
    var err: linux.E = undefined;

    // ---------------------------------------------------------------------
    // Stage 1: mmap
    // ---------------------------------------------------------------------
    timer.reset();

    const addr = linux.mmap(
        null,
        MAP_SIZE,
        linux.PROT.READ | linux.PROT.WRITE,
        .{ .ANONYMOUS = true, .TYPE = .PRIVATE },
        -1,
        0,
    );
    err = linux.E.init(addr);
    if (err != .SUCCESS) std.debug.panic("err={}", .{err});

    const mmap_ns = timer.read();

    // ---------------------------------------------------------------------
    // Stage 2: mlockall
    // ---------------------------------------------------------------------
    var mlock_ns: u64 = 0;

    var flags: u32 = 0;
    if (lock.current) flags |= 1;
    if (lock.future) flags |= 2;
    if (lock.on_fault) flags |= 4;

    timer.reset();
    if (flags != 0) {
        const result = linux.syscall1(.mlockall, flags);
        err = linux.E.init(result);
        if (err != .SUCCESS) std.debug.panic("err={}", .{err});
    }
    mlock_ns = timer.read();

    // ---------------------------------------------------------------------
    // Stage 3: touch first byte of every page
    // ---------------------------------------------------------------------
    timer.reset();

    const base: [*]u8 = @ptrFromInt(addr);
    var offset: usize = 0;
    while (offset < MAP_SIZE) : (offset += PAGE_SIZE) {
        base[offset] = 0xAA;
    }

    const touch_ns = timer.read();

    // ---------------------------------------------------------------------
    // Reporting
    // ---------------------------------------------------------------------
    const out = std.io.getStdOut().writer();

    try out.print("mmap 16GiB:      {}\n", .{std.fmt.fmtDuration(mmap_ns)});
    try out.print("mlockall:        {}\n", .{std.fmt.fmtDuration(mlock_ns)});
    try out.print("page touching:   {}\n", .{std.fmt.fmtDuration(touch_ns)});
}
