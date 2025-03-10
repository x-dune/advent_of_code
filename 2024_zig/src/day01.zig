const std = @import("std");

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    var input = std.ArrayList([]u8).init(std.heap.page_allocator);
    var buffer: [1024]u8 = undefined;
    while (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        try input.append(line);
    }

    for (input.items, 0..) |item, i| {
        if (i == 5) {
            break;
        }
        std.debug.print("{s}\n", .{item});
    }

    return;
}
