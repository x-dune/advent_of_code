const std = @import("std");

pub fn main() void {
    const allocator = std.heap.page_allocator;
    _ = allocator; // autofix
    const stdin = std.io.getStdIn().reader();

    // Create a buffer to store input
    var buffer: [1024]u8 = undefined;

    // Read a line from stdin
    const result = stdin.readUntilDelimiterOrEof(&buffer, '\n');

    // Handle the result of the read operation
    const input = result catch |err| {
        std.debug.print("Error reading input: {}\n", .{err});
        return;
    };

    // If input is null (error occurred), return
    if (input == null) {
        std.debug.print("No input read or error occurred\n", .{});
        return;
    }

    // Now that we know input is not null, we can access its length
    const input_slice = input orelse &[]u8{}; // Fallback to empty slice if null
    const bytes_read = input_slice.len;

    // Null-terminate the string for safe handling
    buffer[bytes_read] = 0;

    // Create a slice from the buffer with the valid data read
    const input_slice_from_buffer = buffer[0..bytes_read];

    // Print the input slice as a string
    std.debug.print("You entered: {}\n", .{input_slice_from_buffer});
}
