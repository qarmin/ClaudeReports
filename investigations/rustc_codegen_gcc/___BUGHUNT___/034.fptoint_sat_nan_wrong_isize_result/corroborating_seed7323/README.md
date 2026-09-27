Supplementary material, NOT a separate candidate. See ../RESULT.md's "Corroborating evidence
(seed 7323)" section for the full writeup. `repro.rs` is a rustlantis-generated custom_mir program
(seed 7323 from this session's 2h fuzzing campaign), reduced from 6564 to 6557 lines by removing 7
dead/decoy raw-pointer writes that were flagged as genuine Undefined Behavior under
`MIRIFLAGS=-Zmiri-tree-borrows cargo miri run` (Tree Borrows, the model rustlantis actually
validates under - see candidate 034's own RESULT.md for why plain/default Miri is the wrong
oracle here) - NOT further hand-minimized beyond that due to time constraints (ddmin on the
remaining lines did not converge in the time available). Still reproduces the LLVM-vs-GCC
mismatch and is clean under Tree Borrows. Root-caused to the exact same `Builder::fptoint_sat`
NaN-to-`isize::MIN` mechanism as candidate 034's own 181-line repro, reached via a different,
simpler-looking value-flow (`_22 = _12 - _12` where `_12` is an infinite `f32`, producing NaN,
then `_14 = _22 as isize`).
