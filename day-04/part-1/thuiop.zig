const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn Grid(comptime T: type) type {
    return struct {
        array: []T,
        width: usize,
        height: usize,
        default: T,

        pub fn from_input(input: []const u8, allocator: std.mem.Allocator) Grid(bool) {
            var array = allocator.alloc(bool, input.len) catch unreachable; 
            const width = std.mem.indexOfScalar(u8, input, "\n"[0]) orelse unreachable;
            for (input, 0..) |value, i| {
                array[i] = value == "@"[0];
            }
            return .{
                .array = array,
                .width = width,
                .height = width,
                .default = false,
            };
        }

        pub fn get(self: Grid(T), i: usize, j: usize) T {
            if ((i > self.width-1) or (i < 0) or (j > self.height-1) or (j < 0)) {
                return self.default;
            }
            else {
                return self.array[j+(self.width+1)*i];
            }
        }

        pub fn set(self: Grid(T), i: usize, j: usize, val: T) void {
            if (~((i > self.width-1) or (i < 0) or (j > self.height-1) or (j < 0))) {
                self.array[j+(self.width+1)*i] = val;
            }
        }

        pub fn apply_kernel(self: Grid(bool), i: usize, j: usize, kernel: []const T, kernel_width: usize) usize {
            var count: usize = 0;
            for (0..kernel_width) |k_i| {
                for (0..kernel_width) |k_j| {
                    count += @intFromBool(kernel[k_i+kernel_width*k_j] and self.get(i+k_i-kernel_width/2, j+k_j-kernel_width/2));
                }
            }
            return count;
        }
    };
}

fn run(input: [:0]const u8) i64 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings
    defer arena.deinit(); // clear memory
    const allocator = arena.allocator();

    const grid = Grid(bool).from_input(input,allocator);

    const kernel: [9]bool = .{true, true, true, true, false, true, true, true ,true};
    const kernel_width = 3;
    var count: i64 = 0;
    for (0..grid.width) |i| {
        for (0..grid.height) |j| {
            if (grid.get(i, j) and grid.apply_kernel(i, j, &kernel, kernel_width) < 4) {
                count += 1;
            }
        }
    }

    return count;
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
