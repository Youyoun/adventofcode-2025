const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn is_invalid(n: usize) bool {
    var max_exp: usize = 10;
    var max_pow: usize = 1;
    while (max_exp <= n) {
        max_exp *= 10;
        max_pow += 1;
    }
    max_exp = std.math.pow(usize, 10, max_pow / 2);

    var base_exp: usize = 10;
    var base_pow: usize = 1;
    outer: while (base_exp <= max_exp) {
        if ((max_pow / base_pow) * base_pow != max_pow) {
            base_exp *= 10;
            base_pow += 1;
            continue;
        }
        const possible_num = n % base_exp;
        var exp = base_exp;
        while (exp <= n) {
            if ((n / exp) % base_exp != possible_num) {
                base_exp *= 10;
                base_pow += 1;
                continue :outer;
            }
            exp *= base_exp;
        }
        return true;
    }
    return false;
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
