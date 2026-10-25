use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use epaint::{Pos2, Shape, Stroke};

fn run_case(case: &str) {
    let path = [Pos2::new(0.0, 0.0), Pos2::new(100.0, 0.0)];
    let stroke = Stroke::new(1.0, epaint::Color32::WHITE);
    let shapes = match case {
        "control" => Shape::dashed_line(&path, stroke, 5.0, 5.0),
        "zero" => Shape::dashed_line(&path, stroke, 0.0, 0.0),
        "empty" => Shape::dashed_line_with_offset(&path, stroke, &[], &[], 0.0),
        _ => unreachable!(),
    };
    println!("{case}: returned {} shapes", shapes.len());
}

fn main() {
    if let Some(case) = std::env::args().nth(1) {
        run_case(&case);
        return;
    }
    for case in ["control", "zero", "empty"] {
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
            if start.elapsed() > Duration::from_secs(5) {
                child.kill().unwrap();
                break "STILL RUNNING after 5s, killed (hang)".to_owned();
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        println!("{case}: {status}");
    }
}
