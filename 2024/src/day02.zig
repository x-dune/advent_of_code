const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);
    const allocator = gpa.allocator();

    var reports = std.ArrayList(std.ArrayList(i64)).init(allocator);
    defer {
        for (reports.items) |x| {
            x.deinit();
        }
        reports.deinit();
    }

    const stdin = std.io.getStdIn().reader();
    while (try stdin.readUntilDelimiterOrEofAlloc(allocator, '\n', 1024)) |line| {
        defer allocator.free(line);
        var it = std.mem.splitScalar(u8, line, ' ');

        var levels = std.ArrayList(i64).init(allocator);

        while (it.next()) |x| {
            const level = try std.fmt.parseInt(i64, x, 10);
            try levels.append(level);
        }
        try reports.append(levels);
    }

    // for (reports.items) |levels| {
    //     for (levels.items[0..1]) |x| {
    //         std.debug.print("{} ", .{x});
    //     }
    //     std.debug.print("\n", .{});
    // }

    var answer1: i64 = 0;

    for (reports.items) |levels| {
        const is_ascending = levels.items[0] < levels.items[1];
        var valid = true;
        for (levels.items[0 .. levels.items.len - 1], levels.items[1..]) |x, y| {
            const diff = y - x;
            // std.debug.print(" xxx {} xxx ", .{diff});
            if ((is_ascending and diff < 1 or diff > 3) or (!is_ascending and diff > -1 or diff < -3)) {
                valid = false;
            }
            // std.debug.print("{} ", .{x});
        }
        if (valid) answer1 += 1;
        // std.debug.print("- {} - {}\n", .{ is_ascending, valid });
    }

    std.debug.print("{}\n", .{answer1});
}
