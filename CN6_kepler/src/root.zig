const std = @import("std");

pub fn read_input(comptime file_path: []const u8, allocator: std.mem.Allocator) ![]u8 {
    return try std.fs.cwd().readFileAlloc(allocator, file_path, 25_000);
}

pub fn to_rad(deg: f64) f64 {
    return deg * (std.math.pi) / 180.0;
}

/// convert floats array
pub fn farrA2arrB(comptime A: type, comptime B: type, arrA: std.ArrayList(A), allocator: std.mem.Allocator) !std.ArrayList(B) {
    var arrB = try std.ArrayList(B).initCapacity(allocator, arrA.items.len);
    for (arrA.items) |f| arrB.appendAssumeCapacity(@as(B, @floatCast(f)));
    return arrB;
}

pub fn print_arr(comptime T: type, arr: std.ArrayList(T)) void {
    std.debug.print("[", .{});
    for (arr.items, 0..) |item, i| {
        if (i == arr.items.len - 1) {
            std.debug.print("{}", .{item});
        } else {
            std.debug.print("{}, ", .{item});
        }
    }
    std.debug.print("]\n", .{});
}

pub fn compute_triangle_area(a: f64, b: f64, c: f64) f64 {
    const p = (a + b + c) / 2.0;
    return @sqrt(p * (p - a) * (p - b) * (p - c));
}

pub fn distance(a: f64, b: f64, c: f64) f64 {
    return @sqrt(a * a + b * b + c * c);
}
