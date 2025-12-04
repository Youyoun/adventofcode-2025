const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) i64 {
    var it = std.mem.splitScalar(u8, input, "\n"[0]);
    var joltage: i64 = 0;
    while (it.next()) |bank| {
        var first_index = bank.len - 2;
        var second_index = bank.len - 1;
        var i = bank.len - 3;
        while (i < 10000000) : (i -= 1) {
            if (bank[i] >= bank[first_index]) {
                if (bank[first_index] >= bank[second_index]) {
                    second_index = first_index;
                }
                first_index = i;
            }
        }
        joltage += (bank[first_index] - 48) * 10 + bank[second_index] - 48;
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
