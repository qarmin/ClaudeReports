use orx_parallel::*;
use std::collections::BTreeMap;

// Single root, splitting at depth 1 into two spines (A at local width 0, B at local
// width 1) that each keep expanding with branching factor BRANCH every level; only the
// last local child (index BRANCH-1) of a spine continues to the next round, the rest
// terminate immediately (still visited/output, just not expanded further). Because both
// spines descend from the very same root (no cross-root ambiguity), apply the exact same
// recurrence with the same per-level radix (coef(d) = BRANCH, since both lineages
// jointly determine max_width at every level), and A's own compounded path key starts
// strictly smaller than B's (local width 0 < 1 at the branch point) and stays strictly
// smaller at every depth as long as no overflow occurs, the following invariant must
// hold under the documented `IterationOrder::Ordered` (default) for every depth d: every
// A-lineage element visited at depth d must sort before every B-lineage element visited
// at the same depth d.
//
// BRANCH/LEVELS chosen so the encoded usize sort key overflows partway through
// (BRANCH^LEVELS ~ 3^42 ~ 1.09e20 >> usize::MAX ~ 1.8e19 on 64-bit), while total node
// count stays tiny (~2 spines * LEVELS * BRANCH = ~250 nodes total, so this runs in
// milliseconds - the bug is about the *encoded sort key* overflowing, not about needing
// to actually visit an astronomically large tree).
const BRANCH: usize = 3;
const LEVELS: usize = 42;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Branch {
    Root,
    A,
    B,
}

fn main() {
    let visited: Vec<_> = par_recursive(
        [(0usize, BRANCH - 1, Branch::Root)],
        |&(depth, id, branch)| match branch {
            Branch::Root => vec![(1, BRANCH - 1, Branch::A), (1, BRANCH - 1, Branch::B)],
            Branch::A | Branch::B if depth < LEVELS && id == BRANCH - 1 => {
                (0..BRANCH).map(move |i| (depth + 1, i, branch)).collect()
            }
            _ => Vec::new(),
        },
    )
    .collect();

    println!("total visited = {}", visited.len());

    let mut last_a_at_depth: BTreeMap<usize, usize> = BTreeMap::new();
    let mut first_b_at_depth: BTreeMap<usize, usize> = BTreeMap::new();
    for (idx, &(depth, _id, branch)) in visited.iter().enumerate() {
        match branch {
            Branch::A => {
                last_a_at_depth.insert(depth, idx);
            }
            Branch::B => {
                first_b_at_depth.entry(depth).or_insert(idx);
            }
            Branch::Root => {}
        }
    }

    let mut violations = 0;
    for (&depth, &last_a) in &last_a_at_depth {
        if let Some(&first_b) = first_b_at_depth.get(&depth) {
            if last_a > first_b {
                violations += 1;
                println!(
                    "ORDER VIOLATED at tree depth {depth}: an A element at output index {last_a} \
                     appears after a B element at output index {first_b} (both same depth, \
                     A should sort first since A's path key started smaller)"
                );
            }
        }
    }
    if violations == 0 {
        println!("ORDER OK: within every depth level, all A elements precede all B elements");
        std::process::exit(0);
    } else {
        println!("TOTAL VIOLATIONS: {violations}");
        std::process::exit(1);
    }
}
