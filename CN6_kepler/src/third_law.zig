const std = @import("std");
const root = @import("root.zig");

pub const ThirdLaw = struct {
    /// radius of earth
    const UA: f64 = 149_597_870_700;

    /// a year into seconds
    const YEAR: f64 = 365.35 * 24 * 60 * 60;

    /// Allocator used by the ThirdLaw when requesting memory.
    allocator: std.mem.Allocator,

    T: std.ArrayList(f64),
    a: std.ArrayList(f64),

    size: usize,

    pub fn init(datas: []u8, allocator: std.mem.Allocator, size: usize) !ThirdLaw {
        var T = try std.ArrayList(f64).initCapacity(allocator, size);
        var a = try std.ArrayList(f64).initCapacity(allocator, size);
        // in case something went wrong
        errdefer {
            T.deinit();
            a.deinit();
        }

        var lines = std.mem.splitSequence(u8, datas, "\n");
        while (lines.next()) |line| {
            var linedata = std.mem.splitSequence(u8, line, " ");

            var datalist = [2]f64{ 0.0, 0.0 };
            for (0..2) |i| datalist[i] = try std.fmt.parseFloat(f64, linedata.next().?);

            const adata = datalist[0] * UA; // convertion from UA to m
            const Tdata = datalist[1] * YEAR; // convertion from year to s

            a.appendAssumeCapacity(adata);
            T.appendAssumeCapacity(Tdata);
        }

        std.debug.assert(T.items.len == size);
        std.debug.assert(a.items.len == size);

        return ThirdLaw{
            .allocator = allocator,
            .T = T,
            .a = a,
            .size = size,
        };
    }

    pub fn compute_a_cube(self: ThirdLaw) !std.ArrayList(f32) {
        var a_cube = try std.ArrayList(f64).initCapacity(self.allocator, self.size);
        defer a_cube.deinit();
        for (self.a.items) |aval| a_cube.appendAssumeCapacity(aval * aval * aval / 1e6);
        return root.farrA2arrB(f64, f32, a_cube, self.allocator);
    }

    pub fn compute_T_square(self: ThirdLaw) !std.ArrayList(f32) {
        var Tsquare = try std.ArrayList(f64).initCapacity(self.allocator, self.size);
        defer Tsquare.deinit();
        for (self.T.items) |Tval| Tsquare.appendAssumeCapacity(Tval * Tval / 1e6);
        return root.farrA2arrB(f64, f32, Tsquare, self.allocator);
    }

    /// Frees all associated memory.
    pub fn deinit(self: *ThirdLaw) void {
        self.T.deinit();
        self.a.deinit();
        self.* = undefined;
    }
};
