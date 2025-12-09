const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings
    defer arena.deinit(); // clear memory

    a = arena.allocator();

    var it = std.mem.splitScalar(u8, input, "\n"[0]);
    const first_line = it.next() orelse unreachable;
    var beams: []usize = a.alloc(usize, first_line.len) catch unreachable;
    @memset(beams, 0);
    beams[std.mem.indexOfScalar(u8, first_line, "S"[0]) orelse unreachable] = 1;
    var n_split: usize = 0;
    while (it.next()) |line| {
        for (0..beams.len) |i| {
            if (line[i] == "^"[0] and beams[i] == 1) {
                beams[i] = 0;
                beams[i+1] = 1;
                beams[i-1] = 1;
                n_split += 1;
            }
        }
    }

    return n_split;
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
