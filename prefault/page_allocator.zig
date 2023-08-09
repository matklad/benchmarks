// Copy-pasted from Zig stdlib

const std = @import("std");
const builtin = @import("builtin");
const Allocator = std.mem.Allocator;
const mem = std.mem;
const os = std.os;
const maxInt = std.math.maxInt;
const assert = std.debug.assert;

const VTable = struct {
    alloc: *const fn (n: usize) ?[]u8,
    free: *const fn (slice: []u8) void,
};

pub fn page_allocator(comptime options: struct { populate: bool }) VTable {
    return struct {
        fn alloc(n: usize) ?[]u8 {
            assert(n > 0);
            if (n > maxInt(usize) - (mem.page_size - 1)) return null;
            const aligned_len = mem.alignForward(usize, n, mem.page_size);

            const hint = @atomicLoad(@TypeOf(std.heap.next_mmap_addr_hint), &std.heap.next_mmap_addr_hint, .Unordered);
            const slice = os.mmap(
                hint,
                aligned_len,
                os.PROT.READ | os.PROT.WRITE,
                os.MAP.PRIVATE | os.MAP.ANONYMOUS | if (options.populate) os.MAP.POPULATE else 0, // <- here's the change
                -1,
                0,
            ) catch return null;
            assert(mem.isAligned(@intFromPtr(slice.ptr), mem.page_size));
            const new_hint: [*]align(mem.page_size) u8 = @alignCast(slice.ptr + aligned_len);
            _ = @cmpxchgStrong(@TypeOf(std.heap.next_mmap_addr_hint), &std.heap.next_mmap_addr_hint, hint, new_hint, .Monotonic, .Monotonic);
            return slice;
        }

        fn free(slice: []u8) void {
            const buf_aligned_len = mem.alignForward(usize, slice.len, mem.page_size);
            os.munmap(@alignCast(slice.ptr[0..buf_aligned_len]));
        }

        const vtable = VTable{
            .alloc = &alloc,
            .free = &free,
        };
    }.vtable;
}
