use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use epaint::{
    Color32, Pos2, Rect, Shape, Stroke, TessellationOptions, Tessellator, Vec2, pos2, vec2,
    ClippedShape, Primitive,
};

fn run_case(radius: f32) {
    let ellipse = Shape::ellipse_stroke(pos2(100.0, 100.0), Vec2::splat(radius), Stroke::new(1.0, Color32::RED));
    let clip = Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0));
    let mut tessellator = Tessellator::new(1.0, TessellationOptions::default(), [1024, 1024], vec![]);
    let start = Instant::now();
    let prims = tessellator.tessellate_shapes(vec![ClippedShape::new(clip, ellipse)]);
    let vertices: usize = prims
        .iter()
        .map(|p| if let Primitive::Mesh(m) = &p.primitive { m.vertices.len() } else { 0 })
        .sum();
    println!("radius {radius:e}: {vertices} vertices in {:?}", start.elapsed());
}

fn main() {
    if let Some(radius) = std::env::args().nth(1) {
        run_case(radius.parse().unwrap());
        return;
    }
    // A circle of radius 100 for comparison: ~ 6 vertices per ... see below
    for radius in ["100", "1e4", "1e6", "1e8", "1e9"] {
        let mut child = Command::new(std::env::current_exe().unwrap())
            .arg(radius)
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break format!("exited: {status}");
            }
            if start.elapsed() > Duration::from_secs(20) {
                child.kill().unwrap();
                break "STILL RUNNING after 20s, killed".to_owned();
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        println!("radius {radius}: {status}");
    }
}
