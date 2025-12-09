const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Tile = struct {
    x: i64,
    y: i64,
};

fn run(input: [:0]const u8) u64 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings
    defer arena.deinit(); // clear memory
    a = arena.allocator();

    var it = std.mem.splitScalar(u8, input, "\n"[0]);
    var tiles = std.ArrayList(Tile).init(a);

    while (it.next()) |line| {
        const index = std.mem.indexOfScalar(u8, line, ","[0]) orelse unreachable;
        tiles.append(Tile{
            .x = std.fmt.parseInt(i64, line[0..index], 10) catch unreachable,
            .y = std.fmt.parseInt(i64, line[index+1..], 10) catch unreachable,
        }) catch unreachable;
    }

    var max: u64 = 0;
    for (tiles.items, 0..) |tile, i| {
        for (i+1..tiles.items.len) |j| {
            const tile2 = tiles.items[j];
            max = @max(max,(@abs(tile.x - tile2.x)+1) * (@abs(tile.y - tile2.y)+1));
        }
    }
    return max;
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
