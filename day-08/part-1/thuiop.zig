const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Point = struct {
    x: usize,
    y: usize,
    z: usize,

    fn from_str(line: []const u8) Point {
        var it = std.mem.splitScalar(u8, line, ","[0]);
        return Point{
            .x = std.fmt.parseInt(usize, it.next() orelse unreachable, 10) catch unreachable,
            .y = std.fmt.parseInt(usize, it.next() orelse unreachable, 10) catch unreachable,
            .z = std.fmt.parseInt(usize, it.next() orelse unreachable, 10) catch unreachable,
        };
    }

    fn distance(self: Point, other: Point) usize {
        return (self.x - other.x)*(self.x - other.x)
             + (self.y - other.y)*(self.y - other.y)
             + (self.z - other.z)*(self.z - other.z);
    }
};

const Distance = struct {
    point_id1: usize,
    point_id2: usize,
    distance: usize,

    fn order(ctx: void, d1: Distance, d2: Distance) std.math.Order {
        _ = ctx;
        return std.math.order(d1.distance, d2.distance);
    }
};

fn run(input: [:0]const u8) usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings defer arena.deinit(); // clear memory
    a = arena.allocator();

    var it = std.mem.splitScalar(u8, input, "\n"[0]);
    var points = std.ArrayList(Point).init(a);
    while (it.next()) |line| {
        points.append(Point.from_str(line)) catch unreachable;
    }

    var distances = std.PriorityQueue(Distance, void, Distance.order).init(a, {});
    for (points.items, 0..) |point1, i| {
        for (i+1..points.items.len) |j| {
            const point2 = points.items[j];
            distances.add(Distance{
                .point_id1 = i,
                .point_id2 = j,
                .distance = point1.distance(point2)
            }) catch unreachable;

        }
    }

    var circuits: []usize = a.alloc(usize, points.items.len) catch unreachable;
    for (0..circuits.len) |i| {
        circuits[i] = i;
    }

    for (0..1000) |_| {
        const dist = distances.remove();
        const circuit1 = circuits[dist.point_id1];
        const circuit2 = circuits[dist.point_id2];
        if (circuit1 != circuit2) {
            for (0..circuits.len) |i| {
                if (circuits[i] == circuit1) {
                    circuits[i] = circuit2;
                }
            }
        } 
    }


    var circuit_counts: []usize = a.alloc(usize, points.items.len) catch unreachable;
    for (0..circuit_counts.len) |i| {
        circuit_counts[circuits[i]] += 1;
    }
    std.mem.sort(usize,circuit_counts,{},comptime std.sort.desc(usize));
    return circuit_counts[0] * circuit_counts[1] * circuit_counts[2];
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings defer arena.deinit(); // clear memory

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
