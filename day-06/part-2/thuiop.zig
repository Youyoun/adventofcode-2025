const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings
    defer arena.deinit(); // clear memory

    a = arena.allocator();

    var it = std.mem.splitScalar(u8, input, "\n"[0]);

    const first_line = it.next() orelse unreachable;

    const num_col: usize = first_line.len;
    var num_row: usize = 1;
    while (it.next()) |_| {
        num_row += 1;
    }
    num_row -= 1;
    it.reset();

    for(0..num_row) |_| {
        _ = it.next();
    }
    var symbols = std.ArrayList(u8).init(a);
    const last_line = it.next() orelse unreachable;
    var it3 = std.mem.splitScalar(u8, last_line, " "[0]);
    while (it3.next()) |val| {
        if (val.len == 0) {
            continue;
        }
        symbols.append(val[0]) catch unreachable;
    }

    var grand_total: usize = 0;
    var current_numbers = std.ArrayList(usize).init(a);
    var count: usize = 0;
    for (0..num_col) |n_col| {
        const num_str = a.alloc(u8, num_row) catch unreachable;
        for (0..num_row) |n_row| {
            num_str[n_row] = input[n_col+(num_col+1)*n_row];
        }
        const num = std.fmt.parseInt(usize, std.mem.trim(u8, num_str, " "), 10) catch 0;
        if (num == 0) {
            var total: usize = current_numbers.items[0];
            for (current_numbers.items[1..]) |n|{
                if (symbols.items[count] == "+"[0]) {
                    total += n;
                }
                else if (symbols.items[count] == "*"[0]) {
                    total *= n;
                }
            }
            grand_total += total;
            count += 1;
            current_numbers.clearRetainingCapacity();
        }
        else {
            current_numbers.append(num) catch unreachable;
        }
    }
    var total: usize = current_numbers.items[0];
    for (current_numbers.items[1..]) |n|{
        if (symbols.items[count] == "+"[0]) {
            total += n;
        }
        else if (symbols.items[count] == "*"[0]) {
            total *= n;
        }
    }
    grand_total += total;

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
