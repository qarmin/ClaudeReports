#!/usr/bin/env python3
"""Switch kurbo_bench's `fit` scene onto the new `fit_to_bezpath_into` API.

This is a HARNESS edit, not a kurbo edit - it is deliberately kept out of patch.diff (which only
ever touches the tested project's own source). It exists so `just bench-reuse` can measure what a
downstream caller that reuses one output `BezPath` actually gains, which is the whole point of the
new API: allocs/iter 1173 -> 0 on the `fit` scene.

Usage: python3 harness_reuse.py <path to kurbo_bench/src/main.rs>
"""

import sys

OLD = '''        "fit" => {
            let cubics = gen_cubics(500, 13);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    let src = OffsetCurve { c: *c, d: 3.0 };
                    let out = fit_to_bezpath(&src, 1e-3);
                    h.path(&out);
                }
                black_box(h.0)
            })
        }'''

NEW = '''        "fit" => {
            let cubics = gen_cubics(500, 13);
            let mut out = BezPath::new();
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    let src = OffsetCurve { c: *c, d: 3.0 };
                    kurbo::fit_to_bezpath_into(&src, 1e-3, &mut out);
                    h.path(&out);
                }
                black_box(h.0)
            })
        }'''


def main() -> None:
    path = sys.argv[1]
    with open(path, encoding="utf-8") as f:
        src = f.read()
    if NEW in src:
        print("harness already switched to fit_to_bezpath_into")
        return
    if OLD not in src:
        sys.exit(f"the `fit` scene in {path} does not match the expected text; harness drifted")
    with open(path, "w", encoding="utf-8") as f:
        f.write(src.replace(OLD, NEW))
    print("harness `fit` scene switched to fit_to_bezpath_into with a reused BezPath")


if __name__ == "__main__":
    main()
