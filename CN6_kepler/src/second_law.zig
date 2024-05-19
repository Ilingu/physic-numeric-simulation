const std = @import("std");
const root = @import("root.zig");

pub const SecondLaw = struct {
    /// radius of earth
    const RT: f64 = 6378;

    /// Allocator used by the SecondLaw when requesting memory.
    allocator: std.mem.Allocator,

    t: std.ArrayList(f64), // could have been [22]usize but I want a flexible program
    x: std.ArrayList(f64),
    y: std.ArrayList(f64),
    z: std.ArrayList(f64),

    size: usize,

    pub fn init(datas: []u8, allocator: std.mem.Allocator, size: usize) !SecondLaw {
        var t = try std.ArrayList(f64).initCapacity(allocator, size);
        var x = try std.ArrayList(f64).initCapacity(allocator, size);
        var y = try std.ArrayList(f64).initCapacity(allocator, size);
        var z = try std.ArrayList(f64).initCapacity(allocator, size);
        // in case something went wrong
        errdefer {
            t.deinit();
            x.deinit();
            y.deinit();
            z.deinit();
        }

        var lines = std.mem.splitSequence(u8, datas, "\n");
        _ = lines.next();
        while (lines.next()) |line| {
            var linedata = std.mem.splitSequence(u8, line, " ");

            var datalist = try std.ArrayList(f64).initCapacity(allocator, 4);
            defer datalist.deinit();

            while (linedata.next()) |data|
                datalist.appendAssumeCapacity(try std.fmt.parseFloat(f64, data));

            const time = datalist.items[0] * 60; // convertion from minutes to seconds
            const r: f64 = (datalist.items[3] + RT) * 1000;
            const theta: f64 = root.to_rad(datalist.items[2]);
            const phi: f64 = root.to_rad(datalist.items[1]);

            const xdata: f64 = r * @sin(theta) * @cos(phi);
            const ydata: f64 = r * @sin(theta) * @sin(phi);
            const zdata: f64 = r * @cos(theta);

            t.appendAssumeCapacity(time);
            x.appendAssumeCapacity(xdata);
            y.appendAssumeCapacity(ydata);
            z.appendAssumeCapacity(zdata);
        }

        std.debug.assert(t.items.len == size);
        std.debug.assert(x.items.len == size);
        std.debug.assert(y.items.len == size);
        std.debug.assert(z.items.len == size);

        return SecondLaw{
            .allocator = allocator,
            .t = t,
            .x = x,
            .y = y,
            .z = z,
            .size = size,
        };
    }

    /// Frees all associated memory.
    pub fn deinit(self: *SecondLaw) void {
        self.t.deinit();
        self.x.deinit();
        self.y.deinit();
        self.z.deinit();
        self.* = undefined;
    }

    pub fn compute_radius(self: SecondLaw) !std.ArrayList(f32) {
        var radius = try std.ArrayList(f64).initCapacity(self.allocator, self.size);
        defer radius.deinit();

        for (0..self.size) |i| {
            const x = self.x.items[i];
            const y = self.y.items[i];
            const z = self.z.items[i];

            radius.appendAssumeCapacity(root.distance(x, y, z));
        }

        return try root.farrA2arrB(f64, f32, radius, self.allocator);
    }

    pub fn compute_velocities(self: SecondLaw) !std.ArrayList(f32) {
        var v = try std.ArrayList(f64).initCapacity(self.allocator, self.size);
        defer v.deinit();

        // compute first value with an O(t) (Forward Difference)
        {
            const delta_t = self.t.items[1] - self.t.items[0];
            const vx = (self.x.items[1] - self.x.items[0]) / delta_t;
            const vy = (self.y.items[1] - self.y.items[0]) / delta_t;
            const vz = (self.z.items[1] - self.z.items[0]) / delta_t;
            v.appendAssumeCapacity(root.distance(vx, vy, vz));
        }
        // compute middle values with an O(t^2) (Central Difference)
        for (1..(self.size - 1)) |i| {
            const delta_t = self.t.items[i + 1] - self.t.items[i - 1];
            const vx = (self.x.items[i + 1] - self.x.items[i - 1]) / (2 * delta_t);
            const vy = (self.y.items[i + 1] - self.y.items[i - 1]) / (2 * delta_t);
            const vz = (self.z.items[i + 1] - self.z.items[i - 1]) / (2 * delta_t);
            v.appendAssumeCapacity(root.distance(vx, vy, vz));
        }
        // compute last value with an O(t) (Backward Difference)
        {
            const last_i = self.x.items.len - 1;
            const delta_t = self.t.items[last_i] - self.t.items[last_i - 1];
            const vx = (self.x.items[last_i] - self.x.items[last_i - 1]) / delta_t;
            const vy = (self.y.items[last_i] - self.y.items[last_i - 1]) / delta_t;
            const vz = (self.z.items[last_i] - self.z.items[last_i - 1]) / delta_t;
            v.appendAssumeCapacity(root.distance(vx, vy, vz));
        }

        return try root.farrA2arrB(f64, f32, v, self.allocator);
    }

    pub fn compute_areal_velocities(self: SecondLaw) !std.ArrayList(f32) {
        var v_area = try std.ArrayList(f64).initCapacity(self.allocator, self.size);
        defer v_area.deinit();

        // compute first value with an O(t) (Forward Difference)
        {
            const delta_t = self.t.items[1] - self.t.items[0];
            const va = blk: {
                const a = root.distance(self.x.items[0], self.y.items[0], self.z.items[0]);
                const b = root.distance(self.x.items[1], self.y.items[1], self.z.items[1]);
                const c = root.distance(self.x.items[1] - self.x.items[0], self.y.items[1] - self.y.items[0], self.z.items[1] - self.z.items[0]);
                break :blk root.compute_triangle_area(a, b, c);
            };
            v_area.appendAssumeCapacity(va / delta_t);
        }
        // compute middle values with an O(t^2) (Central Difference)
        for (1..(self.size - 1)) |i| {
            const delta_t = self.t.items[i + 1] - self.t.items[i - 1];
            const va = blk: {
                const a = root.distance(self.x.items[i - 1], self.y.items[i - 1], self.z.items[i - 1]);
                const b = root.distance(self.x.items[i + 1], self.y.items[i + 1], self.z.items[i + 1]);
                const c = root.distance(self.x.items[i + 1] - self.x.items[i - 1], self.y.items[i + 1] - self.y.items[i - 1], self.z.items[i + 1] - self.z.items[i - 1]);
                break :blk root.compute_triangle_area(a, b, c);
            };
            v_area.appendAssumeCapacity(va / delta_t);
        }
        // compute last value with an O(t) (Backward Difference)
        {
            const last_i = self.x.items.len - 1;
            const delta_t = self.t.items[last_i] - self.t.items[last_i - 1];
            const va = blk: {
                const a = root.distance(self.x.items[last_i - 1], self.y.items[last_i - 1], self.z.items[last_i - 1]);
                const b = root.distance(self.x.items[last_i], self.y.items[last_i], self.z.items[last_i]);
                const c = root.distance(self.x.items[last_i] - self.x.items[last_i - 1], self.y.items[last_i] - self.y.items[last_i - 1], self.z.items[last_i] - self.z.items[last_i - 1]);
                break :blk root.compute_triangle_area(a, b, c);
            };
            v_area.appendAssumeCapacity(va / delta_t);
        }

        return try root.farrA2arrB(f64, f32, v_area, self.allocator);
    }
};
