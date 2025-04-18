const std = @import("std");

fn isValid(items: []const i64) bool {
    var valid = true;
    var all_inc = true;
    var all_dec = true;
    for (items[0 .. items.len - 1], items[1..]) |x, y| {
        const diff = y - x;
        if (diff > 0) {
            all_inc = false;
        }
        if (diff < 0) {
            all_dec = false;
        }
        if (@abs(diff) < 1 or @abs(diff) > 3) {
            valid = false;
            break;
        }
    }
    return valid and (all_inc or all_dec);
}

fn validWithoutIndex(allocator: std.mem.Allocator, items: []i64, index: usize) !bool {
    var new_items = std.ArrayList(i64).init(allocator);
    defer new_items.deinit();
    for (items, 0..) |item, i| {
        if (i != index) try new_items.append(item);
    }
    return isValid(new_items.items);
}

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

    var answer1: i64 = 0;
    var answer2: i64 = 0;

    for (reports.items) |levels| {
        if (isValid(levels.items)) {
            answer1 += 1;
            answer2 += 1;
            continue;
        }
        for (0..levels.items.len) |i| {
            if (try validWithoutIndex(allocator, levels.items, i)) {
                answer2 += 1;
                break;
            }
        }
    }

    std.debug.print("{}\n{}\n", .{ answer1, answer2 });
}
