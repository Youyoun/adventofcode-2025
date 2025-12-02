const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn is_invalid(n: usize) bool {
    var i: usize = 1;
    var exp: usize = 10;
    while (exp < n) {
        exp *= 10;
        i += 1;
    }
    if (i % 2 == 1) {
        return false;
    }
    else {
        const half_exp = std.math.pow(usize, 10, i / 2);
        return n % half_exp == @divFloor(n, half_exp);
    }
}

fn run(input: [:0]const u8) u64 {
    var it = std.mem.splitScalar(u8, input, ","[0]);
    var n_invalid: u64 = 0;
    while (it.next()) |range| {
        var it2 = std.mem.splitScalar(u8, range, "-"[0]);
        const beg = std.fmt.parseInt(usize, it2.next() orelse unreachable, 10) catch unreachable;
        const end = std.fmt.parseInt(usize, it2.next() orelse unreachable, 10) catch unreachable;
        for (beg..end+1) |i| {
            if (is_invalid(i)) {
                n_invalid += i;
            }
        }
        
    }
    return n_invalid;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings
    defer arena.deinit(); // clear memory

    a = arena.allocator();

    var arg_it = try std.process.argsWithAllocator(a);
    _ = arg_it.skip(); // skip over exe name

    const input: [:0]const u8 = arg_it.next().?;

    const start: i128 = std.time.nanoTimestamp(); // start time
    const answer = run(input); // compute answer
    const end: i128 = std.time.nanoTimestamp();

    const elapsed_nano: f64 = @floatFromInt(end - start);
    const elapsed_milli = elapsed_nano / 1_000_000.0;
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}
