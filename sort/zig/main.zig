const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile("input.data", .{});
    defer file.close();

    // Read the contents
    var data = try file.readToEndAllocOptions(
        allocator,
        64 * 1024 * 1024,
        64 * 1024 * 1024,
        @alignOf(u128),
        null,
    );
    defer allocator.free(data);
    std.debug.assert(data.len == 64 * 1024 * 1024);

    const array = std.mem.bytesAsSlice([8]u128, data);

    const attempts = 10;
    var t = try std.time.Timer.start();
    var i: u8 = 0;
    while (i < attempts) : (i += 1) {
        const k = i % 8;
        std.sort.sort([8]u128, array, k, struct {
            fn lessThan(kk: u8, x: [8]u128, y: [8]u128) bool {
                return x[kk] < y[kk];
            }
        }.lessThan);
    }
    const nanos = t.lap();

    std.debug.print(
        \\fingerprint={}
        \\time  = {}
        \\
    , .{
        array[0][0],
        std.fmt.fmtDuration(nanos),
    });
}

// fn main() -> std::io::Result<()> {
//     let mut data = std::fs::read("./input.data")?;
//     assert_eq!(data.len(), 64 * 1024 * 1024);

//     let data = unsafe {
//         let ptr = data.as_mut_ptr();
//         let len = data.len() / 128;
//         std::slice::from_raw_parts_mut::<[u128; 8]>(ptr.cast(), len)
//     };

//     let attempts = 10;
//     let t = std::time::Instant::now();
//     for i in 0..attempts {
//         let k = i % 8;
//         data.sort_by_key(|it| it[k])
//     }

//     let t = t.elapsed();
//     println!("fingerprint = {}\ntime = {t:0.2?}", data[0][0],);
//     Ok(())
// }
