const std = @import("std");

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    var input = std.ArrayList([]u8).init(std.heap.page_allocator);
    // var buffer: [20000]u8 = undefined;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    while (try stdin.readUntilDelimiterOrEofAlloc(allocator, '\n', 2000)) |line| {
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
