const std = @import("std");
const assert = std.debug.assert;
const log = std.log;

pub fn main() !void {
    const page_allocator = @import("./page_allocator.zig").page_allocator;
    const arg = if (std.os.argv.len == 2)
        std.mem.sliceTo(std.os.argv[1], 0)
    else
        @panic("one argument required");
    const allocator = if (std.mem.eql(u8, arg, "--populate"))
        page_allocator(.{ .populate = true })
    else if (std.mem.eql(u8, arg, "--no-populate"))
        page_allocator(.{ .populate = false })
    else
        @panic("--populate or --no-populate required");

    const page_count: u32 = 10 * 1024 * 1024;
    const page_size: u64 = std.mem.page_size;

    const size = page_count * page_size;

    log.info("allocating {}MiB", .{size / 1024 / 1024});

    var t = try std.time.Timer.start();
    const buffer = allocator.alloc(size) orelse @panic("OOM");

    log.info("alloc: {}μs", .{t.lap() / std.time.ns_per_ms});

    t.reset();
    var hash: u8 = 97;
    for (0..page_count) |page| {
        buffer[page * page_size] = hash;
        hash = hash *% hash;
    }
    log.info("touch: {}μs", .{t.lap() / std.time.ns_per_ms});

    t.reset();
    var total: u32 = 0;
    for (0..page_count) |page| {
        total ^= buffer[page * page_size];
    }
    log.info("read:  {}μs sideffect={X}", .{ t.lap() / std.time.ns_per_ms, total });

    t.reset();
    allocator.free(buffer);
    log.info("free:  {}μs", .{t.lap() / std.time.ns_per_ms});
}
