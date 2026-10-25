
#[test]
fn tmp_rounded_rect_normals_equivalence() {
    let mut seed = 99_u64;
    let mut rnd = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as f32) / (u32::MAX >> 1) as f32
    };
    let mut max_diff = 0.0_f32;
    let mut typ = 0.0_f32;
    let mut n_used = 0;
    for _ in 0..200_000 {
        let w = 4.0 + rnd() * 400.0;
        let h = 4.0 + rnd() * 400.0;
        let x = rnd() * 4000.0 - 1000.0;
        let y = rnd() * 4000.0 - 1000.0;
        let r = 0.5 + rnd() * rnd() * 80.0;
        let rect = Rect::from_min_size(pos2(x, y), vec2(w, h));
        let mut pts = Vec::new();
        path::rounded_rectangle(&mut pts, RoundedRect::new(rect, r));
        let Some(normals) = rounded_rect_normals(pts.len()) else { continue };
        let cr = RoundedRect::new(rect, r).corner_radius();
        if !(cr.nw == cr.ne && cr.nw == cr.sw && cr.nw == cr.se && 0.5 <= cr.nw) { continue; }
        n_used += 1;
        let mut p = Path::default();
        p.add_line_loop(&pts);
        for (a, n) in p.0.iter().zip(normals) {
            let d = (a.normal.x - n.x).abs().max((a.normal.y - n.y).abs());
            max_diff = max_diff.max(d);
            if r >= 2.0 && x.abs() < 1000.0 && y.abs() < 1000.0 { typ = typ.max(d); }
        }
    }
    eprintln!("used {n_used} max normal diff {max_diff:e} typical(r>=2,|xy|<1000) {typ:e}");
}
