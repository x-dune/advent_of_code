const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);
    const allocator = gpa.allocator();

    var left = std.ArrayList(u64).init(allocator);
    defer left.deinit();
    var right = std.ArrayList(u64).init(allocator);
    defer right.deinit();

    const stdin = std.io.getStdIn().reader();
    while (try stdin.readUntilDelimiterOrEofAlloc(allocator, '\n', 2000)) |line| {
        defer allocator.free(line);
        var it = std.mem.splitSequence(u8, line, "   ");

        const left_number = try std.fmt.parseInt(u64, it.next().?, 10);
        const right_number = try std.fmt.parseInt(u64, it.next().?, 10);

        try left.append(left_number);
        try right.append(right_number);
    }

    std.mem.sort(u64, left.items, {}, std.sort.asc(u64));
    std.mem.sort(u64, right.items, {}, std.sort.asc(u64));

    var answer1: u64 = 0;
    for (left.items, right.items) |l, r| {
        const temp_l: i64 = @intCast(l);
        const temp_r: i64 = @intCast(r);
        answer1 += @abs(temp_l - temp_r);
    }

    var count = std.AutoHashMap(u64, u64).init(allocator);
    defer count.deinit();
    for (right.items) |r| {
        const v = count.get(r) orelse 0;
        try count.put(r, v + 1);
    }
    var answer2: u64 = 0;
    for (left.items) |l| {
        const v = count.get((l)) orelse continue;
        answer2 += l * v;
    }

    std.debug.print("{}\n{}\n", .{ answer1, answer2 });

    return;
}
