const std = @import("std");

const plotlib = @import("plotlib");
const Scatter = plotlib.Scatter;
const Line = plotlib.Line;

/// The type of an RGB color.
pub const RGB = u48;

pub const DrawPlot = struct {
    figure: plotlib.Figure,

    const SIZE = 800;

    pub fn init(title: []const u8, allocator: std.mem.Allocator) !DrawPlot {
        const figure = plotlib.Figure.init(allocator, .{ .title = .{ .text = title }, .width = .{ .pixel = SIZE }, .height = .{ .pixel = SIZE } });

        return DrawPlot{ .figure = figure };
    }

    /// Frees all associated memory.
    pub fn deinit(self: *DrawPlot) void {
        self.figure.deinit();
        self.* = undefined;
    }

    pub fn plot(self: *DrawPlot, title: []const u8, x: std.ArrayList(f32), y: std.ArrayList(f32), color: RGB) !void {
        std.debug.assert(x.items.len == y.items.len);
        try self.figure.addPlot(Line{ .x = x.items, .y = y.items, .style = .{ .color = color, .title = title } });
    }

    pub fn draw_and_save(self: *DrawPlot, path: []const u8) !void {
        var svg = try self.figure.show();
        defer svg.deinit();

        // Write to an output file (out.svg)
        var file = try std.fs.cwd().createFile(path, .{});
        defer file.close();

        try svg.writeTo(file.writer());
    }
};
