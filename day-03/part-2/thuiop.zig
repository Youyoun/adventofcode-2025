const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) usize {
    var it = std.mem.splitScalar(u8, input, "\n"[0]);
    var joltage: usize = 0;
    while (it.next()) |bank| {
        var indexes: [12]usize = undefined;
        for (0..12) |i| {
            indexes[11-i] = bank.len - 1 - i;
        }
        var i = bank.len - 1 - 12;
        while (i < 10000000) : (i -= 1) {
            if (bank[i] >= bank[indexes[0]]) {
                var prev_index = indexes[0];
                for (0..11) |j| {
                    if (bank[prev_index] >= bank[indexes[j+1]]) {
                        const current_index = indexes[j+1];
                        indexes[j+1] = prev_index;
                        prev_index = current_index;
                    }
                    else {
                        break;
                    }
                }
                indexes[0] = i;
            }
        }
        var exp: usize = 1;
        for (0..12) |j| {
            joltage += (bank[indexes[11-j]] - 48) * exp;
            exp *= 10;
        }
    }
    return joltage;
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
