const std = @import("std");
const assert = std.debug.assert;

pub const Account = extern struct {
    id: u128,
    debits_pending: u128 = 0,
    debits_posted: u128 = 0,
    credits_pending: u128 = 0,
    credits_posted: u128 = 0,
    user_data_128: u128 = 0,
    user_data_64: u64 = 0,
    user_data_32: u32 = 0,
    reserved: u32 = 0,
    ledger: u32 = 0,
    code: u16 = 0,
    flags: u16 = 0,
    timestamp: u64 = 0,
};

const ctx: struct {
    pub fn hash(_: @This(), a: Account) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, key_fn(a));
        return hasher.final();
    }
    pub fn eql(_: @This(), a: Account, b: Account) bool {
        return key_fn(a) == key_fn(b);
    }
} = .{};

const ctx2: struct {
    pub fn hash(_: @This(), a: u128) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, key_fn(a));
        return hasher.final();
    }
    pub fn eql(_: @This(), a: u128, b: Account) bool {
        return key_fn(a) == key_fn(b);
    }
} = .{};

fn key_fn(a: anytype) u128 {
    return switch (@TypeOf(a)) {
        u128 => a,
        Account => a.id,
        else => @compileError("bad key"),
    };
}

pub const Transfer = extern struct {
    id: u128 = 0,
    debit_account_id: u128,
    credit_account_id: u128,
    amount: u128 = 0,
    pending_id: u128 = 0,
    user_data_128: u128 = 0,
    user_data_64: u64 = 0,
    user_data_32: u32 = 0,
    timeout: u32 = 0,
    ledger: u32 = 0,
    code: u16 = 0,
    flags: u16 = 0,
    timestamp: u64 = 0,
};

pub fn main() !void {
    var allocator_gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    const allocator = allocator_gpa.allocator();

    // const random_seed = std.crypto.random.int(u64);
    const random_seed = 92;
    var random_default = std.rand.DefaultPrng.init(random_seed);
    const random = random_default.random();

    const batch_size = 8_000;
    const batch_count = 1000;
    const cache_size = 100_000_000;
    const attempt_count = 5;

    const accounts: []Account = try allocator.alloc(Account, cache_size);
    defer allocator.free(accounts);

    const batches: [][]Transfer = try allocator.alloc([]Transfer, batch_count);
    defer allocator.free(batches);

    const transfers: []Transfer = try allocator.alloc(Transfer, batch_count * batch_size);
    defer allocator.free(transfers);
    for (batches, 0..) |*batch, batch_index| {
        batch.* = transfers[batch_index * batch_size ..][0..batch_size];
    }

    generate_accounts(random, accounts);
    generate_batches(random, accounts, batches);

    std.debug.print("big   size = {}KiB\n", .{@sizeOf(Account) * cache_size / 1024});
    std.debug.print("small size = {}Kib\n", .{@sizeOf(Account) * batch_size / 1024});

    std.debug.print("big hash map\n", .{});
    for (0..attempt_count) |_| {
        try big_map(allocator, accounts, batches);
    }

    std.debug.print("big hash map / small hash map\n", .{});
    for (0..attempt_count) |_| {
        try big_map_small_map(allocator, accounts, batches);
    }
}

/// Half of the accounts are the same, half of the remaining half are the same, etc.
fn generate_accounts(rng: std.Random, accounts: []Account) void {
    const pareto = true;
    if (pareto) {
        var p: u32 = 1;
        var index: u32 = 0;
        outer: while (true) : (p = p * 2) {
            const id = rng.int(u128);
            for (0..p) |_| {
                accounts[index] = .{ .id = id };
                index += 1;
                if (index == accounts.len) break :outer;
            }
        }
    } else {
        for (accounts) |*account| {
            account.* = .{ .id = rng.int(u128) };
        }
    }
    rng.shuffle(Account, accounts);
}

fn generate_batches(
    rng: std.Random,
    accounts: []const Account,
    batches: []const []Transfer,
) void {
    for (batches) |batch| generate_transfers(rng, accounts, batch);
}

fn generate_transfers(rng: std.Random, accounts: []const Account, transfers: []Transfer) void {
    for (0..transfers.len) |index| {
        const cr_account_index = rng.intRangeLessThan(usize, 0, accounts.len);
        const dr_account_index = while (true) {
            const candidate_index = rng.intRangeLessThan(usize, 0, accounts.len);
            if (accounts[candidate_index].id != accounts[cr_account_index].id) {
                break candidate_index;
            }
        } else unreachable;
        transfers[index] = .{
            .credit_account_id = accounts[cr_account_index].id,
            .debit_account_id = accounts[dr_account_index].id,
            .amount = rng.intRangeLessThan(u128, 0, 1_000_000),
        };
        assert(transfers[index].credit_account_id != transfers[index].debit_account_id);
    }
}

fn big_map(
    allocator: std.mem.Allocator,
    accounts: []const Account,
    batches: []const []const Transfer,
) !void {
    var map: std.HashMapUnmanaged(
        Account,
        void,
        @TypeOf(ctx),
        std.hash_map.default_max_load_percentage,
    ) = .{};
    defer map.deinit(allocator);
    try map.ensureTotalCapacity(allocator, @intCast(accounts.len));
    for (accounts) |*account| map.putAssumeCapacity(account.*, {});

    var timer = try std.time.Timer.start();
    for (batches) |batch| {
        for (batch) |*transfer| {
            map.getKeyPtrAdapted(transfer.credit_account_id, ctx2).?.credits_posted += transfer.amount;
            map.getKeyPtrAdapted(transfer.debit_account_id, ctx2).?.debits_posted += transfer.amount;
        }
    }

    const validation = map.getKeyPtrAdapted(accounts[0].id, ctx2).?.credits_posted;
    std.debug.print("elapsed={} checksum={}\n", .{
        std.fmt.fmtDuration(timer.lap()),
        validation,
    });
}

fn big_map_small_map(
    allocator: std.mem.Allocator,
    accounts: []const Account,
    batches: []const []const Transfer,
) !void {
    var map_big: std.HashMapUnmanaged(
        Account,
        void,
        @TypeOf(ctx),
        std.hash_map.default_max_load_percentage,
    ) = .{};
    var map_small: std.HashMapUnmanaged(
        Account,
        void,
        @TypeOf(ctx),
        std.hash_map.default_max_load_percentage,
    ) = .{};
    defer {
        map_small.deinit(allocator);
        map_big.deinit(allocator);
    }

    try map_big.ensureTotalCapacity(allocator, @intCast(accounts.len));
    try map_small.ensureTotalCapacity(allocator, @intCast(batches[0].len)); // assume eqi-sized batches.
    for (accounts) |*account| map_big.putAssumeCapacity(account.*, {});

    var timer = try std.time.Timer.start();
    for (batches) |batch| {
        assert(map_small.count() == 0);
        for (batch) |*transfer| {
            if (map_small.getKeyPtrAdapted(transfer.credit_account_id, ctx2)) |account| {
                // Hot path.
                account.credits_posted += transfer.amount;
            } else {
                // Cold path.
                map_small.getOrPutAssumeCapacity(
                    map_big.getKeyAdapted(transfer.credit_account_id, ctx2).?,
                ).key_ptr.credits_posted += transfer.amount;
            }
            if (map_small.getKeyPtrAdapted(transfer.debit_account_id, ctx2)) |account| {
                account.debits_posted += transfer.amount;
            } else {
                map_small.getOrPutAssumeCapacity(
                    map_big.getKeyAdapted(transfer.debit_account_id, ctx2).?,
                ).key_ptr.*.debits_posted += transfer.amount;
            }
        }
        var it = map_small.keyIterator();
        while (it.next()) |account| {
            map_big.getKeyPtrAdapted(account.id, ctx2).?.* = account.*;
        }
        map_small.clearRetainingCapacity();
    }

    const validation = map_big.getKeyPtrAdapted(accounts[0].id, ctx2).?.credits_posted;
    std.debug.print("elapsed={} checksum={}\n", .{
        std.fmt.fmtDuration(timer.lap()),
        validation,
    });
}
