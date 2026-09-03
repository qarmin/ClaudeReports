#!/usr/bin/env python3
"""
Differential check for candidate 079 (COMPOSE_PLUSLIGHTER_REUSES_PLUS_FORMULA).

Compares, over random + boundary samples in the valid [0,1] domain:
  (A) the CPU highp "Plus" formula as coded in
      sparse_strips/vello_cpu/src/fine/highp/compose.rs (also used, unmodified,
      for Compose::PlusLighter):
          result = clamp(src_premul + bg_premul, 0.0, 1.0)   (applied to R,G,B,A alike,
          since fa=fb=1.0 for Plus and the alpha channel is treated the same as R/G/B
          by the shared macro)
  (B) the project's own separately-derived, dedicated PlusLighter formula, found in
      sparse_strips/vello_sparse_shaders/shaders/helpers/blend.wesl (COMPOSE_PLUS_LIGHTER
      case) and vello_shaders/shader/shared/blend.wgsl (COMPOSE_PLUS_LIGHTER case),
      both textually identical:
          co  = min(1.0, as_ * cs + ab * cb)
          out_a = min(1.0, as_ + ab)
      where cs/cb are separated (non-premultiplied) colors and as_/ab the two alphas.
"""
import random

def cpu_plus_formula(src_premul, bg_premul):
    # per-channel: clamp(a + b, 0, 1), applied uniformly to R,G,B,A
    return tuple(max(0.0, min(1.0, a + b)) for a, b in zip(src_premul, bg_premul))

def gpu_pluslighter_formula(cs, as_, cb, ab):
    co = tuple(min(1.0, as_ * s + ab * b) for s, b in zip(cs, cb))
    out_a = min(1.0, as_ + ab)
    return co + (out_a,)

def to_premul(c, a):
    return tuple(ch * a for ch in c) + (a,)

def run_case(cs, as_, cb, ab):
    src_premul = to_premul(cs, as_)
    bg_premul = to_premul(cb, ab)
    a_res = cpu_plus_formula(src_premul, bg_premul)
    b_res = gpu_pluslighter_formula(cs, as_, cb, ab)
    return a_res, b_res

max_abs_diff = 0.0
mismatches = 0
n = 0

boundary_vals = [0.0, 1.0, 0.5, 1e-6, 1 - 1e-6]
cases = []
# Boundary-value grid (0, 1, mid, near-0, near-1) across as_, ab and per-channel colors.
for as_ in boundary_vals:
    for ab in boundary_vals:
        for cs_v in boundary_vals:
            for cb_v in boundary_vals:
                cs = (cs_v, 1 - cs_v, cs_v)
                cb = (cb_v, cb_v, 1 - cb_v)
                cases.append((cs, as_, cb, ab))

# Random fuzz on top of the boundary grid.
random.seed(12345)
for _ in range(200000):
    as_ = random.random()
    ab = random.random()
    cs = (random.random(), random.random(), random.random())
    cb = (random.random(), random.random(), random.random())
    cases.append((cs, as_, cb, ab))

for cs, as_, cb, ab in cases:
    n += 1
    a_res, b_res = run_case(cs, as_, cb, ab)
    diff = max(abs(x - y) for x, y in zip(a_res, b_res))
    max_abs_diff = max(max_abs_diff, diff)
    if diff > 1e-9:
        mismatches += 1
        if mismatches <= 5:
            print(f"MISMATCH cs={cs} as_={as_} cb={cb} ab={ab} -> A={a_res} B={b_res} diff={diff}")

print(f"Total cases checked: {n}")
print(f"Mismatches (diff > 1e-9): {mismatches}")
print(f"Max abs per-channel diff observed: {max_abs_diff}")
