use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use epaint::{Color32, CubicBezierShape, QuadraticBezierShape, Stroke, pos2};

fn run_case(case: &str) {
    let none = Stroke::NONE;
    let n = match case {
        // endpoints with different x: the documented default tolerance works
        "cubic_control" => {
            CubicBezierShape::from_points_stroke([pos2(0.0, 0.0), pos2(30.0, 10.0), pos2(-30.0, 20.0), pos2(5.0, 30.0)], false, Color32::TRANSPARENT, none).flatten(None).len()
        }
        // endpoints on the same vertical line
        "cubic_same_x" => {
            CubicBezierShape::from_points_stroke([pos2(0.0, 0.0), pos2(30.0, 10.0), pos2(-30.0, 20.0), pos2(0.0, 30.0)], false, Color32::TRANSPARENT, none).flatten(None).len()
        }
        "quadratic_same_x" => {
            QuadraticBezierShape::from_points_stroke([pos2(0.0, 0.0), pos2(30.0, 15.0), pos2(0.0, 30.0)], false, Color32::TRANSPARENT, none).flatten(None).len()
        }
        _ => unreachable!(),
    };
    println!("{case}: {n} points");
}

fn main() {
    if let Some(case) = std::env::args().nth(1) {
        run_case(&case);
        return;
    }
    for case in ["cubic_control", "cubic_same_x", "quadratic_same_x"] {
        let mut child = Command::new(std::env::current_exe().unwrap())
            .arg(case)
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break format!("exited: {status}");
            }
            if start.elapsed() > Duration::from_secs(10) {
                child.kill().unwrap();
                break "STILL RUNNING after 10s, killed".to_owned();
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        println!("{case}: {status}");
    }
}
