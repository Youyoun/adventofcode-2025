const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) i64 {
    // your code here
    var it = std.mem.splitScalar(u8, input, "\n"[0]);
    var current_dial: i16 = 50;
    var number_of_zeros: i64 = 0;
    while (it.next()) |rot| {
        const val = std.fmt.parseInt(i16, rot[1..], 10) catch unreachable;
        switch (rot[0]) {
            "L"[0] => current_dial -= val, 
            "R"[0] => current_dial += val, 
            else => unreachable, 
        }
        current_dial = @mod(current_dial,100);
        if (current_dial == 0) {
            number_of_zeros += 1;
        }
    }
    return number_of_zeros;
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
