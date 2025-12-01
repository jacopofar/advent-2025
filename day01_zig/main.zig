const std = @import("std");

const gpa = std.heap.wasm_allocator;
pub fn main() !void {
    var stdin = std.fs.File.stdin().reader(&.{});

    var allocating = std.Io.Writer.Allocating.init(gpa);
    const n = try stdin.interface.streamRemaining(&allocating.writer);
    std.log.debug("{} bytes streamed", .{n});

    const data = try allocating.toOwnedSlice();

    const Direction = enum(u2) { L, R };
    const Rotation = struct { Direction, u8, u8 };
    var rotations = try std.ArrayList(Rotation).initCapacity(gpa, 10);
    var it = std.mem.splitScalar(u8, data, '\n');
    while (it.next()) |x| {
        std.debug.print("seeing line {s}\n", .{x});
        const dir = if (x[0] == 'L') Direction.L else Direction.R;
        std.debug.print("direction {}\n", .{dir});
        const amount = try std.fmt.parseInt(u16, x[1..], 10);
        std.debug.print("amount {s}\n", .{x[1..]});
        try rotations.append(gpa, .{ dir, @intCast(@mod(amount, 100)), @intCast(amount / 100) });
    }
    var position: i16 = 50;
    var countExactZero: u16 = 0;
    var countPassedZero: u16 = 0;

    for (rotations.items, 0..) |item, i| {
        var newPosition: i16 = position;
        if (item[0] == Direction.R) {
            newPosition += item[1];
        } else {
            newPosition -= item[1];
        }

        newPosition = @mod(newPosition, 100);
        std.debug.print("[step {d}] now at position {d} -> {d} ({})", .{ i, position, newPosition, item });
        if (newPosition == 0) {
            countExactZero += 1;
        }
        // 360 turns done, no effect on final position
        // but they affect 0-crossing for part 2
        countPassedZero += item[2];
        std.debug.print("complete turns here: {d}", .{item[2]});

        // in both cases ignore if it starts from 0
        // crossed 0 going L
        if (item[0] == Direction.L and item[1] >= position and position != 0) {
            countPassedZero += 1;
            std.debug.print("zero was crossed", .{});
        }
        // crossed 0 going Rs
        if (item[0] == Direction.R and item[1] + position >= 100 and position != 0) {
            countPassedZero += 1;
            std.debug.print("zero was crossed", .{});
        }
        position = newPosition;
    }
    std.debug.print("SOLUTION PART 1: {d}", .{countExactZero});
    std.debug.print("SOLUTION PART 2: {d}", .{countPassedZero});
}
