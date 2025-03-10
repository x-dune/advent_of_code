const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const memory = try allocator.alloc(u8, 5);
    defer allocator.free(memory);
    memory[0] = 1;
    memory[1] = 2;

    for (memory) |x| {
        print("{d}\n", .{x});
    }

    print("{s}", .{"hello world"});
}
