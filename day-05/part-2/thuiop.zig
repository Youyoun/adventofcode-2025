const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Range = struct {
    beg: usize,
    end: usize,

    pub fn is_in(self: Range, val: usize) bool {
        return (self.beg <= val) and (val <= self.end);
    }

    pub fn comp(ctx: void, r1: Range, r2: Range) bool {
        _ = ctx;
        return r1.beg < r2.beg;
    }
};

fn run(input: [:0]const u8) usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings
    defer arena.deinit(); // clear memory

    a = arena.allocator();

    var it = std.mem.splitScalar(u8, input, "\n"[0]);
    var ranges = std.ArrayList(Range).init(a);
    var fresh_count: usize = 0;
    
   while (it.next()) |val| {
        if (val.len == 0) {
            break;
        }
        const dash_index = std.mem.indexOfScalar(u8, val, "-"[0]) orelse unreachable;
        const beg = std.fmt.parseInt(usize, val[0..dash_index], 10) catch unreachable;
        const end = std.fmt.parseInt(usize, val[dash_index+1..], 10) catch unreachable;
        ranges.append(.{
            .beg = beg,
            .end = end,
        }) catch unreachable;
    }

    std.mem.sort(Range, ranges.items, {}, Range.comp);

    var ranges_merged = std.ArrayList(Range).initCapacity(a,ranges.items.len) catch unreachable;
    ranges_merged.append(ranges.items[0]) catch unreachable;
    for (ranges.items[1..]) |r| {
        const prev_r = &ranges_merged.items[ranges_merged.items.len-1];
        if (prev_r.end >= r.beg) {
            prev_r.beg = @min(r.beg,prev_r.beg);
            prev_r.end = @max(r.end,prev_r.end);
        }
        else {
            ranges_merged.append(r) catch unreachable;
        }
    }

    for (ranges_merged.items) |r| {
        fresh_count += r.end - r.beg + 1;
    }

    return fresh_count;
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
