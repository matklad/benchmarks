const std = @import("std");

const N: u64 = 50_000_000;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer if (gpa.deinit()) @panic("leaks");

    const allocator = gpa.allocator();

    const stdout = std.io.getStdOut();
    const out_stream = stdout.writer();

    var m = std.StringHashMap(u64).init(allocator);
    defer {
        var it = m.keyIterator();
        while (it.next()) |key| {
            const s: []const u8 = key.*;
            allocator.free(s);
        }
        m.deinit();
    }

    const t1 = std.time.milliTimestamp();
    var i: u64 = 0;
    while (i < N)  : (i += 1) {
        const s = try std.fmt.allocPrint(allocator, "{d}", .{i});
        errdefer allocator.free(s);

        try m.put(s, i);
    }
    try out_stream.print("{d}\n", .{std.time.milliTimestamp() - t1});

    const t2 = std.time.milliTimestamp();
    var total: u64 = 0;
    i = 0;
    while (i < N)  : (i += 1) {
        const s = try std.fmt.allocPrint(allocator, "{d}", .{i});
        defer allocator.free(s);

        total += m.get(s).?;
    }
    try out_stream.print("{d}\n", .{std.time.milliTimestamp() - t2});
    try out_stream.print("{d}\n", .{total});
}
