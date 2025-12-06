const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings
    defer arena.deinit(); // clear memory

    a = arena.allocator();

    var it = std.mem.splitScalar(u8, input, "\n"[0]);

    const first_line = it.next() orelse unreachable;

    var num_col: usize = 0;
    var it2 = std.mem.splitScalar(u8, first_line, " "[0]);
    while (it2.next()) |val| {
        if (val.len == 0) {
            continue;
        }
        num_col += 1;
    }

    it.reset();

    var num_row: usize = 0;
    while (it.next()) |_| {
        num_row += 1;
    }
    num_row -= 1;
    it.reset();

    var num_array: []usize = a.alloc(usize, num_row*num_col) catch unreachable;

    var count: usize = 0;
    while (it.next()) |line| {
        var it3 = std.mem.splitScalar(u8, line, " "[0]);
        while (it3.next()) |val| {
            if (val.len == 0) {
                continue;
            }
            num_array[count] = std.fmt.parseInt(usize, val, 10) catch unreachable;
            count += 1;
        }
        if (count == num_row * num_col) {
            break;
        }
    }

    count = 0;
    var grand_total: usize = 0;
    const last_line = it.next() orelse unreachable;
    var it3 = std.mem.splitScalar(u8, last_line, " "[0]);
    while (it3.next()) |val| {
        if (val.len == 0) {
            continue;
        }
        
        var total: usize = num_array[count];
        for(1..num_row) |i| {
            if (val[0] == "+"[0]) {
                total += num_array[i*num_col+count];
            }
            else if (val[0] == "*"[0]) {
                total *= num_array[i*num_col+count];
            }
        }
        grand_total += total;

        count += 1;
    }

    return grand_total;
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
