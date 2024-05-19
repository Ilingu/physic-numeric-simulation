const std = @import("std");
const root = @import("root.zig");

const second_law = @import("second_law.zig");
const third_law = @import("third_law.zig");

const graph = @import("graph.zig");

const gpalogging = false;

pub fn main() !void {
    // Get an allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{ .safety = true }){};
    defer _ = gpa.deinit();

    const allocator = blk: {
        if (gpalogging) {
            var logging_alloc = std.heap.loggingAllocator(gpa.allocator());
            break :blk logging_alloc.allocator();
        }
        break :blk gpa.allocator();
    };

    // try part1(allocator);
    try part2(allocator);
}

fn part1(allocator: std.mem.Allocator) !void {
    const datas = try root.read_input("./assets/6.1", allocator);
    defer allocator.free(datas);

    var lawstate = try second_law.SecondLaw.init(datas, allocator, 22);
    defer lawstate.deinit();

    const t32 = try root.farrA2arrB(f64, f32, lawstate.t, allocator);
    defer t32.deinit();

    const radius = try lawstate.compute_radius();
    defer radius.deinit();

    const velocities = try lawstate.compute_velocities();
    defer velocities.deinit();

    const areal_velocities = try lawstate.compute_areal_velocities();
    defer areal_velocities.deinit();

    // root.print_arr(f32, t32);
    // root.print_arr(f32, radius);
    // root.print_arr(f32, velocities);
    // root.print_arr(f32, areal_velocities);

    var pltfig = try graph.DrawPlot.init("Vitesse aréolaire en fonction du temps", allocator);
    defer pltfig.deinit();

    // try pltfig.plot("Rayon", t32, radius, 0xf94800);
    // try pltfig.plot("Vitesse", t32, velocities, 0x0084ff);
    try pltfig.plot("Vitesse aréolaire", t32, areal_velocities, 0xbd00ad);

    try pltfig.draw_and_save("./out/areal_velocity.svg");
}

fn part2(allocator: std.mem.Allocator) !void {
    const datas = try root.read_input("./assets/6.2", allocator);
    defer allocator.free(datas);

    var lawstate = try third_law.ThirdLaw.init(datas, allocator, 8);
    defer lawstate.deinit();

    const a32 = try lawstate.compute_a_cube();
    defer a32.deinit();

    const T32 = try lawstate.compute_T_square();
    defer T32.deinit();

    root.print_arr(f32, a32);
    root.print_arr(f32, T32);

    var pltfig = try graph.DrawPlot.init("a^3 en fonction de T^2", allocator);
    defer pltfig.deinit();

    try pltfig.plot("a^3=f(T^2)", T32, a32, 0x0084ff);
    try pltfig.draw_and_save("./out/third_law.svg");
}
