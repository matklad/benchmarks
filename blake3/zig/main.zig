const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile("input.data", .{});
    defer file.close();

    // Read the contents
    const data = try file.readToEndAlloc(allocator, 64 * 1024 * 1024);
    defer allocator.free(data);
    std.debug.assert(data.len == 64 * 1024 * 1024);

    var res: u32 = 0;
    const attempts = 100;
    var i: usize = 0;
    var t = try std.time.Timer.start();
    while (i < attempts) : (i += 1) {
        var target: [32]u8 = undefined;
        std.crypto.hash.Blake3.hash(data, target[0..], .{});
        res +%= target[i % 32];
    }
    const nanos = t.lap();

    std.debug.print(
        \\fingerprint={}
        \\time  = {}
        \\MiB/S = {}
        \\
    , .{
        res,
        std.fmt.fmtDuration(nanos),
        @floatToInt(u32, @intToFloat(f64, data.len * attempts) / (1024.0 * 1024.0) / (@intToFloat(f64, nanos) / 1_000_000_000)),
    });
}
